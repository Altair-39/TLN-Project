mod cky;
mod grammar;
use crate::cky::cky_parse_jurafsky;
use crate::grammar::CfgJurafsky;

fn main() {
    let grammar = CfgJurafsky::new();

    let test_sentences = vec![
        "I prefer the money",
        "I book the flight",
        "I book the flight through Huston",
    ];

    for sentence in test_sentences {
        if cky_parse_jurafsky(sentence, &grammar) {
            println!("'{}' is grammatically valid!", sentence);
        } else {
            println!("'{}' is NOT grammatically valid.", sentence);
        }
    }
}
