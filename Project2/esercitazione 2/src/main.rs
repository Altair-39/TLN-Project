use inquire::{Select, Text};
use rand::seq::SliceRandom;
use std::{collections::HashMap, fs}; // Import necessary inquire modules

fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| {
            word.to_lowercase()
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_string()
        })
        .collect()
}

// Function to generate bigrams
fn generate_bigrams(tokens: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut bigrams: HashMap<String, Vec<String>> = HashMap::new();

    for window in tokens.windows(2) {
        if let [word1, word2] = window {
            bigrams
                .entry(word1.clone())
                .or_default()
                .push(word2.clone());
        }
    }

    bigrams
}

// Function to generate trigrams
fn generate_trigrams(tokens: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut trigrams: HashMap<String, Vec<String>> = HashMap::new();

    for window in tokens.windows(3) {
        if let [word1, word2, word3] = window {
            trigrams
                .entry(format!("{} {}", word1, word2))
                .or_default()
                .push(word3.clone());
        }
    }

    trigrams
}

// Function to generate new text using bigrams
fn generate_text(
    bigrams: &HashMap<String, Vec<String>>,
    start_word: &str,
    length: usize,
) -> String {
    let mut result = vec![start_word.to_string()];
    let mut current_word = start_word.to_string();

    for _ in 0..length - 1 {
        if let Some(next_words) = bigrams.get(&current_word) {
            let next_word = next_words.choose(&mut rand::thread_rng()).unwrap();
            result.push(next_word.clone());
            current_word = next_word.clone();
        } else {
            break;
        }
    }

    result.join(" ")
}

// Function to generate new text using trigrams with better error handling
fn generate_trigram_text(
    trigrams: &HashMap<String, Vec<String>>,
    start_phrase: &str,
    length: usize,
) -> String {
    let mut result = vec![start_phrase.to_string()];
    let mut current_phrase = start_phrase.to_string();

    // Check if starting phrase exists in the trigram map
    if !trigrams.contains_key(&current_phrase) {
        println!("Starting phrase not found in trigrams.");
        return String::new();
    }

    for _ in 0..length - 1 {
        if let Some(next_words) = trigrams.get(&current_phrase) {
            let next_word = next_words.choose(&mut rand::thread_rng()).unwrap();
            result.push(next_word.clone());
            current_phrase = format!(
                "{} {}",
                current_phrase
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join(" "),
                next_word
            );
        } else {
            break;
        }
    }

    result.join(" ")
}

// Function to read Moby Dick from file and return its contents as a string
fn read_moby_dick() -> String {
    let file_path = "moby-dick.txt"; // Make sure this file exists in your project directory
    fs::read_to_string(file_path).expect("Unable to read Moby Dick file")
}

fn main() {
    // Read the text from Moby Dick
    let moby_dick_text = read_moby_dick();

    // Tokenize the Moby Dick text
    let tokens = tokenize(&moby_dick_text);

    // Generate bigrams and trigrams
    let bigrams = generate_bigrams(tokens.clone());
    let trigrams = generate_trigrams(tokens);

    // Prompt the user for the type of text generation
    let generation_type = Select::new(
        "Which type of text generation would you like to use?",
        ["Bigrams", "Trigrams"].to_vec(),
    )
    .prompt()
    .unwrap();

    // Prompt the user for the starting word or phrase
    let start_input = if generation_type == "Bigrams" {
        Text::new("Enter the starting word for bigrams:")
            .prompt()
            .unwrap()
    } else {
        Text::new("Enter the starting phrase for trigrams:")
            .prompt()
            .unwrap()
    };

    // Prompt the user for the desired length of the generated text
    let length: usize = Text::new("Enter the length of the generated text:")
        .prompt()
        .unwrap()
        .parse()
        .unwrap_or(25); // Default to 25 if parsing fails

    // Generate text based on user choices
    let generated_text = if generation_type == "Bigrams" {
        generate_text(&bigrams, &start_input, length)
    } else {
        generate_trigram_text(&trigrams, &start_input, length)
    };

    // Display the generated text
    println!("\nGenerated Text:\n{}", generated_text);
}
