use crate::grammar::CfgJurafsky;
use crate::grammar::CfgQuenya;
use std::collections::HashSet;

pub fn cky_parse_jurafsky(sentence: &str, grammar: &CfgJurafsky) -> bool {
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

pub fn cky_parse_quenya(sentence: &str, grammar: &CfgQuenya) -> bool {
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
                println!(
                    "Combining table[{}][{}] (left: {:?}) and table[{}][{}] (right: {:?})",
                    i,
                    k,
                    left_set,
                    k + 1,
                    j,
                    right_set
                );
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
