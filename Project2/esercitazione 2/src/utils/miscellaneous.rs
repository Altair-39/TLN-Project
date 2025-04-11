use std::collections::HashMap;

pub fn tokenize(text: &str) -> Vec<String> {
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
                tokens.push(ch.to_string());
            }
        }
    }

    if !word.is_empty() {
        tokens.push(word);
    }

    tokens
}

pub fn format_text(tokens: &[String]) -> String {
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

pub fn apply_temperature(weights: &HashMap<String, usize>, temperature: f64) -> Vec<f64> {
    weights
        .values()
        .map(|&w| (w as f64).powf(1.0 / temperature))
        .collect()
}
