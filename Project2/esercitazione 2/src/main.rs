use inquire::{Select, Text};
use std::{collections::HashMap, fs};

use rand::distributions::WeightedIndex;
use rand::prelude::*;

mod dataset;
mod ngrams;

fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut word = String::new();

    for ch in text.chars() {
        if ch.is_alphanumeric() {
            word.push(ch.to_ascii_lowercase());
        } else {
            if !word.is_empty() {
                tokens.push(word.clone());
                word.clear();
            }
            if !ch.is_whitespace() {
                tokens.push(ch.to_string()); // keep punctuation as separate token
            }
        }
    }

    if !word.is_empty() {
        tokens.push(word);
    }

    tokens
}

fn generate_bigrams(tokens: Vec<String>) -> HashMap<String, HashMap<String, usize>> {
    let mut bigrams: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for window in tokens.windows(2) {
        if let [word1, word2] = window {
            let entry = bigrams.entry(word1.clone()).or_default();
            *entry.entry(word2.clone()).or_insert(0) += 1;
        }
    }

    bigrams
}

fn generate_trigrams(tokens: Vec<String>) -> HashMap<String, HashMap<String, usize>> {
    let mut trigrams: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for window in tokens.windows(3) {
        if let [word1, word2, word3] = window {
            let key = format!("{} {}", word1, word2);
            let entry = trigrams.entry(key).or_default();
            *entry.entry(word3.clone()).or_insert(0) += 1;
        }
    }

    trigrams
}
fn format_text(tokens: &[String]) -> String {
    let mut result = String::new();
    let punct = [".", ",", "!", "?", ";", ":", ")", "]", "}", "”", "’"];

    for (i, token) in tokens.iter().enumerate() {
        if i == 0 || punct.contains(&token.as_str()) {
            result.push_str(token);
        } else {
            result.push(' ');
            result.push_str(token);
        }
    }

    result
}

fn apply_temperature(weights: &HashMap<String, usize>, temperature: f64) -> Vec<f64> {
    weights
        .values()
        .map(|&w| (w as f64).powf(1.0 / temperature))
        .collect()
}

fn generate_text(
    bigrams: &HashMap<String, HashMap<String, usize>>,
    start_word: &str,
    length: usize,
    temperature: f64,
) -> String {
    let mut rng = thread_rng();
    let mut result = vec![start_word.to_string()];
    let mut current_word = start_word.to_string();

    for _ in 0..length - 1 {
        if let Some(next_words_map) = bigrams.get(&current_word) {
            let words: Vec<_> = next_words_map.keys().cloned().collect();
            let weights: Vec<f64> = apply_temperature(next_words_map, temperature);
            let dist = WeightedIndex::new(&weights).unwrap();
            let next_word = words[dist.sample(&mut rng)].clone();
            result.push(next_word.clone());
            current_word = next_word;
        } else {
            break;
        }
    }

    format_text(&result)
}

fn generate_trigram_text(
    trigrams: &HashMap<String, HashMap<String, usize>>,
    start_phrase: &str,
    length: usize,
    temperature: f64,
) -> String {
    let mut rng = thread_rng();
    let mut result = vec![start_phrase.to_string()];
    let mut current_phrase = start_phrase.to_string();

    for _ in 0..length - 1 {
        if let Some(next_words_map) = trigrams.get(&current_phrase) {
            let words: Vec<_> = next_words_map.keys().cloned().collect();
            let weights: Vec<f64> = apply_temperature(next_words_map, temperature);
            let dist = WeightedIndex::new(&weights).unwrap();
            let next_word = words[dist.sample(&mut rng)].clone();
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

    format_text(&result)
}

fn read_moby_dick() -> String {
    let file_path = "moby-dick.txt";
    fs::read_to_string(file_path).expect("Unable to read Moby Dick file")
}

fn main() {
    let moby_dick_text = read_moby_dick();

    let tokens = tokenize(&moby_dick_text);

    let bigrams = generate_bigrams(tokens.clone());
    let trigrams = generate_trigrams(tokens);

    let generation_type = Select::new(
        "Which type of text generation would you like to use?",
        ["Bigrams", "Trigrams"].to_vec(),
    )
    .prompt()
    .unwrap();

    let temp_input =
        Text::new("Enter temperature (e.g., 0.8 = conservative, 1.0 = default, 1.5 = creative):")
            .prompt()
            .unwrap();
    let temperature: f64 = temp_input.parse().unwrap_or(1.0);

    let start_input = if generation_type == "Bigrams" {
        Text::new("Enter the starting word for bigrams:")
            .prompt()
            .unwrap()
    } else {
        Text::new("Enter the starting phrase for trigrams:")
            .prompt()
            .unwrap()
    };

    let length: usize = Text::new("Enter the length of the generated text:")
        .prompt()
        .unwrap()
        .parse()
        .unwrap_or(25);

    let generated_text = if generation_type == "Bigrams" {
        generate_text(&bigrams, &start_input, length, temperature)
    } else {
        generate_trigram_text(&trigrams, &start_input, length, temperature)
    };

    println!("\nGenerated Text:\n{}", generated_text);
}
