use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::collections::HashMap;

use crate::apply_temperature;
use crate::format_text;

pub fn generate_trigrams(tokens: Vec<String>) -> HashMap<String, HashMap<String, usize>> {
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

pub fn generate_trigram_text(
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
