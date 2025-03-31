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

pub fn cky_parse(sentence: &str, grammar: &Cfg, file_path: &str) -> Option<ParseTreeNode> {
    let words: Vec<String> = sentence.split_whitespace().map(String::from).collect();
    let n = words.len();

    if n == 0 {
        return None;
    }

    let word_to_nt: Vec<HashSet<String>> = words
        .iter()
        .map(|word| grammar.get_non_terminals(&[word.clone()]))
        .collect();

    let mut table: Vec<Vec<HashSet<ParseTreeNode>>> = vec![vec![HashSet::new(); n]; n];

    for (i, word) in words.iter().enumerate() {
        for nt in &word_to_nt[i] {
            let leaf_node = ParseTreeNode {
                symbol: nt.clone(),
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
                if table[i][k].is_empty() || table[k + 1][j].is_empty() {
                    continue;
                }

                let mut new_nodes = Vec::new();

                for left in &table[i][k] {
                    for right in &table[k + 1][j] {
                        let production = vec![left.symbol.clone(), right.symbol.clone()];
                        let possible_lhs = grammar.get_non_terminals(&production);

                        for lhs in possible_lhs {
                            new_nodes.push(ParseTreeNode {
                                symbol: lhs,
                                children: vec![left.clone(), right.clone()],
                            });
                        }
                    }
                }

                for node in new_nodes {
                    table[i][j].insert(node);
                }
            }
        }
    }

    table[0][n - 1]
        .iter()
        .find(|node| node.symbol == "S")
        .cloned()
        .inspect(|parse_tree| {
            if !file_path.is_empty() {
                if let Ok(json) = to_string_pretty(parse_tree) {
                    let _ = File::create(file_path)
                        .and_then(|mut file| file.write_all(json.as_bytes()));
                }
            }
        })
}
