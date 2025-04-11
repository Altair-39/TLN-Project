mod mode;
mod ngrams;
mod utils;

use mode::texts::read_texts;
use mode::tweets::read_tweets;

use ngrams::bigrams::*;
use ngrams::trigrams::*;

use utils::miscellaneous::*;
use utils::prompting::*;

fn main() {
    let mode = get_mode();
    let text_content = if mode == "Texts" {
        read_texts()
    } else {
        read_tweets()
    };

    let tokens = tokenize(&text_content);

    let bigrams = generate_bigrams(tokens.clone());
    let trigrams = generate_trigrams(tokens);

    let generation_type = get_generation_type();

    let temperature = get_temperature();

    let start_input = get_start_input(&generation_type);

    let length = get_length();

    let generated_text = if generation_type == "Bigrams" {
        generate_bigrams_text(&bigrams, &start_input, length, temperature)
    } else {
        generate_trigram_text(&trigrams, &start_input, length, temperature)
    };

    println!("\nGenerated Text:\n{}", generated_text);
}
