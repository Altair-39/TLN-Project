
#include "wordnet/wn.h"
#include <ctype.h>
#include <dirent.h>
#include <libxml/parser.h>
#include <libxml/tree.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <time.h>

#define MAX_WORDS 100
#define MAX_SENTENCE_LENGTH 1024
#define NUM_SENTENCES 50
#define MAX_FILES 100

typedef struct {
  char sentence[MAX_SENTENCE_LENGTH];
  char target_word[50];
  char sense_key[50];
} SemcorSentence;

// Function prototypes
void preprocess_text(char *text, char *tokens[MAX_WORDS], int *token_count);
SynsetPtr get_synsets(const char *word, int pos);
int compute_overlap(const char *signature, const char *context);
SynsetPtr simple_lesk(const char *word, const char *context);
void process_semcor_file(const char *filename, SemcorSentence *sentences,
                         int *count);
void get_random_sentences(SemcorSentence *all_sentences, int total_count,
                          SemcorSentence *selected, int select_count);
void load_semcor(const char *path, SemcorSentence *sentences, int *count);

// Tokenize text
void preprocess_text(char *text, char *tokens[MAX_WORDS], int *token_count) {
  char *word;
  char *text_copy = strdup(text);

  if (!text_copy) {
    printf("Memory allocation failed for text copy!\n");
    return;
  }

  word = strtok(text_copy, " .,!?;:\"()\n");
  while (word && *token_count < MAX_WORDS) {
    for (int i = 0; word[i]; i++)
      word[i] = tolower(word[i]);
    tokens[*token_count] = strdup(word);

    if (!tokens[*token_count]) {
      printf("Memory allocation failed for token: '%s'\n", word);
      break;
    }

    (*token_count)++;
    word = strtok(NULL, " .,!?;:\"()\n");
  }

  free(text_copy);
}

// Fetch WordNet synsets for a word
SynsetPtr get_synsets(const char *word, int pos) {
  return findtheinfo_ds((char *)word, pos, ALLSENSES, 0);
}

// Compute overlap between two sets of words
int compute_overlap(const char *signature, const char *context) {
  char *sig_tokens[MAX_WORDS], *ctx_tokens[MAX_WORDS];
  int sig_count = 0, ctx_count = 0, overlap = 0;
  preprocess_text((char *)signature, sig_tokens, &sig_count);
  preprocess_text((char *)context, ctx_tokens, &ctx_count);

  for (int i = 0; i < sig_count; i++) {
    for (int j = 0; j < ctx_count; j++) {
      if (strcmp(sig_tokens[i], ctx_tokens[j]) == 0) {
        overlap++;
        break;
      }
    }
  }

  // Free allocated tokens
  for (int i = 0; i < sig_count; i++)
    free(sig_tokens[i]);
  for (int i = 0; i < ctx_count; i++)
    free(ctx_tokens[i]);

  return overlap;
}

// Simple Lesk Algorithm for Word Sense Disambiguation
SynsetPtr simple_lesk(const char *word, const char *context) {
  SynsetPtr best_sense = NULL;
  int max_overlap = -1;

  for (int pos = NOUN; pos <= VERB; pos++) {
    SynsetPtr synset = get_synsets(word, pos);
    while (synset) {
      int overlap = compute_overlap(synset->defn, context);
      if (overlap > max_overlap) {
        max_overlap = overlap;
        best_sense = synset;
      }
      synset = synset->nextss;
    }
  }

  return best_sense;
}

// Function to extract lemma and sense key from a <wf> tag
void extract_word_info(xmlNode *wf_node, char *lemma, char *sense_key) {
  xmlChar *lemma_attr = xmlGetProp(wf_node, (xmlChar *)"lemma");
  xmlChar *wnsn_attr = xmlGetProp(wf_node, (xmlChar *)"wnsn");

  if (lemma_attr && wnsn_attr) {
    // Safely copy lemma and sense_key to avoid buffer overflows
    strncpy(lemma, (char *)lemma_attr, 50);
    strncpy(sense_key, (char *)wnsn_attr, 50);
    lemma[49] = '\0';     // Ensure null termination
    sense_key[49] = '\0'; // Ensure null termination

    xmlFree(lemma_attr);
    xmlFree(wnsn_attr);
  } else {
    // Ensure both fields are empty if no data is found
    lemma[0] = '\0';
    sense_key[0] = '\0';
  }
}

// Fix malformed XML entities
void fix_xml_entities(char *line) {
  char *ptr = line;
  while ((ptr = strstr(ptr, "&amp;")) != NULL) {
    strncpy(ptr, "&", 1); // Replace "&amp;" with "&"
    ptr++;
  }

  // Handle other entities like &lt;, &gt;, &quot;, etc.
  while ((ptr = strstr(ptr, "&lt;")) != NULL) {
    strncpy(ptr, "<", 1); // Replace "&lt;" with "<"
    ptr++;
  }

  while ((ptr = strstr(ptr, "&gt;")) != NULL) {
    strncpy(ptr, ">", 1); // Replace "&gt;" with ">"
    ptr++;
  }

  while ((ptr = strstr(ptr, "&quot;")) != NULL) {
    strncpy(ptr, "\"", 1); // Replace "&quot;" with "\""
    ptr++;
  }

  while ((ptr = strstr(ptr, "&apos;")) != NULL) {
    strncpy(ptr, "'", 1); // Replace "&apos;" with "'"
    ptr++;
  }
}

// Process file with libxml
void process_semcor_file(const char *filename, SemcorSentence *sentences,
                         int *count) {
  FILE *file = fopen(filename, "r");
  if (!file) {
    printf("Failed to open file: %s\n", filename);
    return;
  }

  char line[MAX_SENTENCE_LENGTH];
  char current_sentence[MAX_SENTENCE_LENGTH] = {0};
  int sentence_index = 0;
  char target_word[50];
  char sense_key[50];

  while (fgets(line, sizeof(line), file)) {
    fix_xml_entities(line); // Fix the XML entities in the line

    if (strstr(line, "<s ")) { // Start of sentence
      memset(current_sentence, 0,
             sizeof(current_sentence)); // Reset current sentence
    }

    // Parse words inside <wf> tag
    if (strstr(line, "<wf cmd=\"done\"")) {
      char lemma[50] = {0};
      char sense[50] = {0};
      extract_word_info(line, lemma, sense);

      // If we have a lemma and sense, include it in the sentence
      if (lemma[0] != '\0') {
        strncat(current_sentence, lemma,
                MAX_SENTENCE_LENGTH - strlen(current_sentence) - 1);
        strncat(current_sentence, " ",
                MAX_SENTENCE_LENGTH - strlen(current_sentence) - 1);

        // Store the target word and sense key for evaluation
        if (*count < NUM_SENTENCES) {
          strncpy(sentences[*count].sentence, current_sentence,
                  MAX_SENTENCE_LENGTH);
          strncpy(sentences[*count].target_word, lemma, 50);
          strncpy(sentences[*count].sense_key, sense, 50);
          (*count)++;
        }
      }
    }

    if (strstr(line, "</s>")) { // End of sentence
      // Finalize sentence and move to next
    }
  }

  fclose(file);
}

// Load all SemCor files from the directory, regardless of extensions
void load_semcor(const char *path, SemcorSentence *sentences, int *count) {
  DIR *dir;
  struct dirent *ent;
  char filenames[MAX_FILES][256];
  int file_count = 0;

  if ((dir = opendir(path)) != NULL) {
    while ((ent = readdir(dir)) != NULL && file_count < MAX_FILES) {
      // Process files (skip directories, e.g., "." and "..")
      if (ent->d_type == DT_REG) {
        snprintf(filenames[file_count], 256, "%s/%s", path, ent->d_name);
        file_count++;
      }
    }
    closedir(dir);
  } else {
    perror("opendir failed");
    return;
  }

  printf("Attempting to load SemCor from: %s\n", path);
  if (file_count == 0) {
    printf("No files found in directory: %s\n", path);
    return;
  }

  // Process files in random order
  for (int i = 0; i < file_count && *count < NUM_SENTENCES; i++) {
    int r = rand() % file_count;
    process_semcor_file(filenames[r], sentences, count);
  }
}

// Select random sentences from all available
void get_random_sentences(SemcorSentence *all_sentences, int total_count,
                          SemcorSentence *selected, int select_count) {
  int *picked = calloc(total_count, sizeof(int));
  int count = 0;

  while (count < select_count && count < total_count) {
    int r = rand() % total_count;
    if (!picked[r]) {
      selected[count] = all_sentences[r];
      picked[r] = 1;
      count++;
    }
  }

  free(picked);
}

int main() {
  // Initialize random seed
  srand(time(NULL));

  // Set WordNet search directory
  setenv("WNSEARCHDIR", "/usr/share/wordnet", 1);

  // Initialize the WordNet database
  if (wninit() != 0) {
    printf("Error initializing WordNet database.\n");
    return 1;
  }

  // Initialize libxml
  xmlInitParser();

  // Load SemCor sentences
  SemcorSentence all_sentences[NUM_SENTENCES * 10]; // Buffer for many sentences
  int total_count = 0;
  load_semcor("semcor/brown1/tagfiles", all_sentences, &total_count);

  if (total_count == 0) {
    printf("No SemCor sentences found. Using sample data.\n");
    strcpy(all_sentences[0].sentence, "The bank gave me a loan.");
    strcpy(all_sentences[0].target_word, "bank");
    strcpy(all_sentences[0].sense_key, "bank%1:17:01::");
    total_count = 1;
  }

  // Select random subset
  SemcorSentence selected[NUM_SENTENCES];
  get_random_sentences(all_sentences, total_count, selected, NUM_SENTENCES);

  printf("Selected %d random sentences\n", NUM_SENTENCES);

  // Evaluate WSD on the sentences
  int correct = 0;
  for (int i = 0; i < NUM_SENTENCES; i++) {
    printf("\nSentence %d: %s\n", i + 1, selected[i].sentence);
    printf("Target word: %s (Sense key: %s)\n", selected[i].target_word,
           selected[i].sense_key);

    // Apply WSD
    SynsetPtr best_sense =
        simple_lesk(selected[i].target_word, selected[i].sentence);

    if (best_sense) {
      printf("Predicted sense: %s - %s\n", best_sense->words[0],
             best_sense->defn);

      // Compare with gold standard (simplified)
      if (strstr(best_sense->defn, "financial") &&
          strstr(selected[i].sense_key, "bank%1:17:")) {
        correct++;
      } else if (strstr(best_sense->defn, "sloping") &&
                 strstr(selected[i].sense_key, "bank%1:17:00")) {
        correct++;
      }
    } else {
      printf("No synset found for '%s'.\n", selected[i].target_word);
    }
  }

  // Calculate and print accuracy
  if (NUM_SENTENCES > 0) {
    float accuracy = (float)correct / NUM_SENTENCES * 100;
    printf("\nAccuracy: %.2f%% (%d/%d)\n", accuracy, correct, NUM_SENTENCES);
  }

  // Cleanup
  xmlCleanupParser();

  return 0;
}
