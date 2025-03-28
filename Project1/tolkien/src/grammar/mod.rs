use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct Cfg {
    rules: HashMap<String, Vec<Vec<String>>>,
}

impl Cfg {
    pub fn new(grammar: &str) -> Self {
        let file_path = format!("rsrc/grammar/{}.json", grammar);

        if !Path::new(&file_path).exists() {
            panic!("Grammar file '{}' does not exist", file_path);
        }

        let json_data = fs::read_to_string(&file_path)
            .unwrap_or_else(|_| panic!("Failed to read JSON file: {}", file_path));

        let rules: HashMap<String, Vec<Vec<String>>> =
            serde_json::from_str(&json_data).expect("Failed to parse JSON");

        Cfg { rules }
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
