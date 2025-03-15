use crate::grammar::CfgJurafsky;
use std::collections::HashMap;
use std::collections::HashSet;

impl CfgJurafsky {
    pub fn new() -> Self {
        let mut rules = HashMap::new();

        rules.insert(
            "S".to_string(),
            vec![
                vec!["NP".to_string(), "VP".to_string()],
                vec!["X1".to_string(), "VP".to_string()],
            ],
        );

        rules.insert(
            "X1".to_string(),
            vec![vec!["Aux".to_string(), "NP".to_string()]],
        );

        rules.insert(
            "Aux".to_string(),
            vec![vec!["does".to_string()], vec!["do".to_string()]],
        );

        rules.insert(
            "NP".to_string(),
            vec![
                vec!["I".to_string()],
                vec!["she".to_string()],
                vec!["me".to_string()],
                vec!["you".to_string()],
                vec!["Huston".to_string()],
                vec!["NWA".to_string()],
                vec!["Det".to_string(), "Nominal".to_string()],
                vec!["Det".to_string(), "Noun".to_string()],
            ],
        );

        rules.insert(
            "Nominal".to_string(),
            vec![
                vec!["morning".to_string()],
                vec!["quick".to_string()],
                vec!["cool".to_string()],
                vec!["adventurous".to_string()],
                vec!["mountain".to_string()],
                vec!["cold".to_string()],
                vec!["Nominal".to_string(), "Noun".to_string()],
                vec!["Nominal".to_string(), "PP".to_string()],
                vec!["X3".to_string(), "Nominal".to_string()],
            ],
        );

        rules.insert(
            "X3".to_string(),
            vec![
                vec!["X3".to_string(), "Nominal".to_string()],
                vec!["morning".to_string()],
                vec!["quick".to_string()],
                vec!["cool".to_string()],
                vec!["adventurous".to_string()],
                vec!["mountain".to_string()],
            ],
        );

        rules.insert(
            "VP".to_string(),
            vec![
                vec!["book".to_string()],
                vec!["include".to_string()],
                vec!["prefer".to_string()],
                vec!["love".to_string()],
                vec!["like".to_string()],
                vec!["drink".to_string()],
                vec!["Verb".to_string(), "Nominal".to_string()],
                vec!["Verb".to_string(), "NP".to_string()],
                vec!["Verb".to_string(), "PP".to_string()],
                vec!["X2".to_string(), "PP".to_string()],
                vec!["VP".to_string(), "PP".to_string()],
                vec!["Adverb".to_string(), "VP".to_string()],
            ],
        );

        rules.insert(
            "X2".to_string(),
            vec![vec!["Verb".to_string(), "NP".to_string()]],
        );

        rules.insert(
            "PP".to_string(),
            vec![vec!["Preposition".to_string(), "NP".to_string()]],
        );

        rules.insert(
            "Det".to_string(),
            vec![
                vec!["that".to_string()],
                vec!["this".to_string()],
                vec!["the".to_string()],
                vec!["a".to_string()],
            ],
        );

        rules.insert(
            "Noun".to_string(),
            vec![
                vec!["book".to_string()],
                vec!["flight".to_string()],
                vec!["meal".to_string()],
                vec!["money".to_string()],
                vec!["day".to_string()],
                vec!["water".to_string()],
            ],
        );

        rules.insert(
            "Verb".to_string(),
            vec![
                vec!["book".to_string()],
                vec!["include".to_string()],
                vec!["prefer".to_string()],
                vec!["love".to_string()],
                vec!["like".to_string()],
                vec!["drink".to_string()],
            ],
        );

        rules.insert(
            "Pronoun".to_string(),
            vec![
                vec!["I".to_string()],
                vec!["she".to_string()],
                vec!["me".to_string()],
                vec!["you".to_string()],
            ],
        );

        rules.insert(
            "Proper-Noun".to_string(),
            vec![vec!["Huston".to_string()], vec!["NWA".to_string()]],
        );

        rules.insert(
            "Preposition".to_string(),
            vec![
                vec!["from".to_string()],
                vec!["to".to_string()],
                vec!["on".to_string()],
                vec!["near".to_string()],
                vec!["through".to_string()],
            ],
        );

        rules.insert(
            "Adverb".to_string(),
            vec![
                vec!["Adv1".to_string(), "Adv2".to_string()],
                vec!["definitely".to_string()],
                vec!["often".to_string()],
                vec!["never".to_string()],
                vec!["really".to_string()],
                vec!["rarely".to_string()],
            ],
        );

        rules.insert(
            "Adv1".to_string(),
            vec![vec!["never".to_string()], vec!["rarely".to_string()]],
        );

        rules.insert("Adv2".to_string(), vec![vec!["ever".to_string()]]);

        CfgJurafsky { rules }
    }

    pub fn get_non_terminals(&self, sequence: &[String]) -> HashSet<String> {
        let mut result = HashSet::new();
        for (lhs, rhs_list) in &self.rules {
            for rhs in rhs_list {
                if *rhs == sequence {
                    result.insert(lhs.clone());
                }
            }
        }
        result
    }
}
