use crate::grammar::CfgQuenya;
use std::collections::HashMap;
use std::collections::HashSet;

impl CfgQuenya {
    pub fn new() -> Self {
        let mut rules = HashMap::new();

        CfgQuenya { rules }
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
