import nltk
from nltk.corpus import semcor, wordnet as wn
from nltk.tokenize import word_tokenize
import random
import tracemalloc
from functools import lru_cache

# Initialize memory tracking
tracemalloc.start()

# Download required NLTK data
nltk.download('semcor', quiet=True)
nltk.download('wordnet', quiet=True)
nltk.download('punkt', quiet=True)
nltk.download('stopwords', quiet=True)


@lru_cache(maxsize=1000)
def preprocess_text(text):
    """Tokenize and remove stopwords from text with caching"""
    stopwords = set(nltk.corpus.stopwords.words('english'))
    tokens = word_tokenize(text.lower())
    return [token for token in tokens if token.isalpha() and token not in stopwords]


@lru_cache(maxsize=5000)
def get_synsets(word, pos=None):
    """Cached synset lookup"""
    return wn.synsets(word, pos) if pos else wn.synsets(word)


def get_context_synsets(word, context):
    """Get synsets only for specific POS tags with caching"""
    pos_tags = ['n', 'v']  # Focus on nouns and verbs
    return sum((get_synsets(word, pos) for pos in pos_tags), [])


def compute_overlap(signature, context):
    """Optimized overlap computation using sets"""
    signature_words = set(preprocess_text(signature))
    context_words = set(preprocess_text(context))
    return len(signature_words & context_words)


def simple_lesk(word, context):
    """Optimized Lesk algorithm with caching"""
    best_sense = None
    max_overlap = -1
    context_preprocessed = ' '.join(
        preprocess_text(context))  # Cache context processing

    for sense in get_context_synsets(word, context):
        # Build signature with definition and examples
        signature = sense.definition()
        signature += ' ' + ' '.join(sense.examples())

        # Add hypernyms and hyponyms definitions
        for rel in sense.hypernyms() + sense.hyponyms():
            signature += ' ' + rel.definition()

        overlap = compute_overlap(signature, context_preprocessed)
        if overlap > max_overlap:
            max_overlap = overlap
            best_sense = sense

    return best_sense


def extract_semcor_phrases(num_phrases=50):
    """Extract random phrases from SemCor with error handling"""
    phrases = []
    try:
        sents = list(semcor.sents())
        random.shuffle(sents)

        for sent in sents[:num_phrases]:
            sent_str = ' '.join(sent)
            candidates = [word for word in preprocess_text(
                sent_str) if get_synsets(word)]
            if candidates:
                target_word = random.choice(candidates)
                phrases.append((sent_str, target_word))
    except Exception as e:
        print(f"Error accessing SemCor: {e}")

    return phrases[:num_phrases]


def evaluate_lesk():
    """Evaluate the Lesk algorithm with memory tracking"""
    phrases = extract_semcor_phrases(50)
    results = []

    for i, (phrase, target_word) in enumerate(phrases, 1):
        print(f"\nPhrase {i}: {phrase[:100]}...")  # Show first 100 chars
        print(f"Target word: {target_word}")

        senses = get_synsets(target_word)
        print(f"Possible senses: {len(senses)}")

        if senses:
            best_sense = simple_lesk(target_word, phrase)
            if best_sense:
                print(f"Selected: {best_sense.name()
                                   } - {best_sense.definition()[:60]}...")
                results.append((target_word, best_sense))
            else:
                print("No sense selected")
                results.append((target_word, None))
        else:
            print("No senses found")
            results.append((target_word, None))

    return results


if __name__ == "__main__":
    print("Running optimized Lesk algorithm on 50 random SemCor phrases...")

    results = evaluate_lesk()
    total = len(results)
    successful = len([r for r in results if r[1]])

    print("\nEvaluation Summary:")
    print(f"Total processed: {total}")
    print(f"Successful: {successful} ({successful/total*100:.1f}%)")

    # Memory analysis
    snapshot = tracemalloc.take_snapshot()
    print("\nMemory Top 10:")
    for stat in snapshot.statistics('lineno')[:10]:
        print(stat)

    # Clear caches to free memory
    preprocess_text.cache_clear()
    get_synsets.cache_clear()
