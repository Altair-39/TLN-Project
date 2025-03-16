use crate::grammar::Cfg;
use serde::Serialize;
use serde_json::to_string_pretty;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]

pub struct ParseTreeNode {
    pub symbol: String,
    pub children: Vec<ParseTreeNode>,
}

pub fn cky_parse(sentence: &str, grammar: &Cfg, file_path: &str) -> bool {
    let words: Vec<String> = sentence.split_whitespace().map(String::from).collect();
    let n = words.len();

    if n == 0 {
        return false;
    }

    let mut table: Vec<Vec<HashSet<ParseTreeNode>>> = vec![vec![HashSet::new(); n]; n];

    for (i, word) in words.iter().enumerate() {
        let non_terminals = grammar.get_non_terminals(&[word.clone()]);

        for nt in non_terminals {
            let leaf_node = ParseTreeNode {
                symbol: nt,
                children: vec![ParseTreeNode {
                    symbol: word.clone(),
                    children: vec![],
                }],
            };
            table[i][i].insert(leaf_node);
        }
    }

    for length in 2..=n {
        for i in 0..=n - length {
            let j = i + length - 1;
            for k in i..j {
                let left_set = table[i][k].clone();
                let right_set = table[k + 1][j].clone();
                for left in &left_set {
                    for right in &right_set {
                        let possible_lhs =
                            grammar.get_non_terminals(&[left.symbol.clone(), right.symbol.clone()]);
                        for lhs in possible_lhs {
                            let parent_node = ParseTreeNode {
                                symbol: lhs,
                                children: vec![left.clone(), right.clone()],
                            };
                            table[i][j].insert(parent_node);
                        }
                    }
                }
            }
        }
    }

    let start_symbol = "S";
    let contains_start = table[0][n - 1]
        .iter()
        .any(|node| node.symbol == start_symbol);

    if contains_start {
        if let Some(parse_tree) = table[0][n - 1]
            .iter()
            .find(|node| node.symbol == start_symbol)
        {
            if let Ok(json) = to_string_pretty(parse_tree) {
                let mut file = File::create(file_path).expect("Failed to create file");
                file.write_all(json.as_bytes())
                    .expect("Failed to write file");
            }
        }
    }

    contains_start
}
