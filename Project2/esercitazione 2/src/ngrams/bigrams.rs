use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::collections::HashMap;

use crate::apply_temperature;
use crate::format_text;

pub fn generate_bigrams(tokens: Vec<String>) -> HashMap<String, HashMap<String, usize>> {
    let mut bigrams: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for window in tokens.windows(2) {
        if let [word1, word2] = window {
            let entry = bigrams.entry(word1.clone()).or_default();
            *entry.entry(word2.clone()).or_insert(0) += 1;
        }
    }

    bigrams
}

pub fn generate_bigrams_text(
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
