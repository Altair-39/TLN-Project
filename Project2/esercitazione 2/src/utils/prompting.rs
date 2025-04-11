use inquire::{Select, Text};
use std::string::String;

pub fn get_mode() -> String {
    Select::new(
        "Which mode would you like to use?",
        ["Texts", "Tweets"].to_vec(),
    )
    .prompt()
    .unwrap()
    .to_string()
}

pub fn get_generation_type() -> String {
    Select::new(
        "Which type of text generation would you like to use?",
        ["Bigrams", "Trigrams"].to_vec(),
    )
    .prompt()
    .unwrap()
    .to_string()
}

pub fn get_temperature() -> f64 {
    let temp_input =
        Text::new("Enter temperature (e.g., 0.8 = conservative, 1.0 = default, 1.5 = creative):")
            .prompt()
            .unwrap();
    temp_input.parse().unwrap_or(1.0)
}

pub fn get_start_input(generation_type: &str) -> String {
    if generation_type == "Bigrams" {
        Text::new("Enter the starting word for bigrams:")
            .prompt()
            .unwrap()
    } else {
        Text::new("Enter the starting phrase for trigrams:")
            .prompt()
            .unwrap()
    }
}

pub fn get_length() -> usize {
    Text::new("Enter the length of the generated text:")
        .prompt()
        .unwrap()
        .parse()
        .unwrap_or(25)
}

pub fn select_book(book_choices: Vec<String>) -> String {
    Select::new("Select a book:", book_choices)
        .prompt()
        .unwrap()
}

pub fn select_tweet_file(tweet_choices: Vec<String>) -> String {
    Select::new("Select a person for tweets:", tweet_choices)
        .prompt()
        .unwrap()
}
