use crate::grammar::Cfg;
use std::collections::HashSet;

pub fn cky_parse(sentence: &str, grammar: &Cfg) -> bool {
    let words: Vec<String> = sentence.split_whitespace().map(String::from).collect();
    let n = words.len();

    if n == 0 {
        return false;
    }

    let mut table: Vec<Vec<HashSet<String>>> = vec![vec![HashSet::new(); n]; n];

    for (i, word) in words.iter().enumerate() {
        table[i][i] = grammar.get_non_terminals(&[word.clone()]);
    }

    for length in 2..=n {
        for i in 0..=n - length {
            let j = i + length - 1;

            for k in i..j {
                let left_set = table[i][k].clone();
                let right_set = table[k + 1][j].clone();

                for b in left_set {
                    for c in &right_set {
                        let possible_lhs = grammar.get_non_terminals(&[b.clone(), c.clone()]);
                        table[i][j].extend(possible_lhs);
                    }
                }
            }
        }
    }

    table[0][n - 1].contains("S")
}
