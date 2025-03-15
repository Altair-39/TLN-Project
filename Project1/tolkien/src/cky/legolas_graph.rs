use std::collections::HashSet;

struct LegolasGraph {
    name: String,
    parents: HashSet<String>,
    children: HashSet<String>,
}

impl LegolasGraph {
    fn new(name: String, parents: Vec<LegolasGraph>, children: Vec<LegolasGraph>) -> Self {
        let mut node = LegolasGraph {
            name,
            parents: HashSet::new(),
            children: HashSet::new(),
        };

        for parent in parents {
            node.add_parent(&parent);
        }

        for child in children {
            node.add_child(&child);
        }

        node
    }

    fn add_parent(&mut self, node: &LegolasGraph) {
        self.parents.insert(node.name.clone());
        if !node.children.contains(&self.name) {
            node.add_child(self);
        }
    }

    fn add_child(&mut self, node: &LegolasGraph) {
        self.children.insert(node.name.clone());
        if !node.parents.contains(&self.name) {
            node.add_parent(self);
        }
    }

    fn print_tree_string(&self) {
        self.print_tree_string_recursive(1);
    }

    fn print_tree_string_recursive(&self, level: usize) {
        println!("{}", self.name);
        for child in &self.children {
            if level != 1 {
                print!("|");
            }
            for _ in 0..(level - 1) {
                print!("    ");
            }
            print!("|____");
            child.print_tree_string_recursive(level + 1);
        }
    }

    fn get_string_rec(&self) -> String {
        if self.children.is_empty() {
            return self.name.clone();
        }
        let mut s = String::new();
        for child in &self.children {
            s = format!("{} {}", child.get_string(), s);
        }
        s
    }

    fn get_string(&self) -> String {
        let s = self.get_string_rec();
        let words: Vec<&str> = s.split_whitespace().collect();
        words.join(" ")
    }
}
