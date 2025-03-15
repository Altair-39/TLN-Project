use crate::grammar::CfgQuenya;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec;

impl CfgQuenya {
    pub fn new() -> Self {
        let mut rules = HashMap::new();

        rules.insert(
            "S".to_string(),
            vec![vec!["NP".to_string(), "VP".to_string()]],
        );

        rules.insert(
            "VP".to_string(),
            vec![
                vec!["Verb".to_string(), "NP".to_string()],
                vec!["Verb".to_string()],
            ],
        );

        rules.insert(
            "NP".to_string(),
            vec![
                vec!["NP".to_string(), "Noun".to_string()],
                vec!["Noun".to_string()],
            ],
        );

        rules.insert(
            "Noun".to_string(),
            vec![
                vec!["hesto".to_string()],
                vec!["macil".to_string()],
                vec!["aran".to_string()],
                vec!["aiwi".to_string()],
                vec!["eldar".to_string()],
                vec!["atan".to_string()],
                vec!["eldan".to_string()],
                vec!["tecil".to_string()],
            ],
        );

        rules.insert(
            "Verb".to_string(),
            vec![
                vec!["same".to_string()],
                vec!["tira".to_string()],
                vec!["Nalme".to_string()],
                vec!["antane".to_string()],
            ],
        );
        CfgQuenya { rules }
    }

    pub fn get_non_terminals(&self, sequence: &[String]) -> HashSet<String> {
        let mut result = HashSet::new();
        for (lhs, rhs_list) in &self.rules {
            for rhs in rhs_list {
                println!("Matching rule: {} → {:?}", lhs, rhs);
                if *rhs == sequence {
                    result.insert(lhs.clone());
                    println!("{:?}", result);
                }
            }
        }
        result
    }
}
