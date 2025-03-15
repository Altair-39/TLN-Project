pub mod jurafsky;
pub mod quenya;
use std::collections::HashMap;

pub struct CfgJurafsky {
    rules: HashMap<String, Vec<Vec<String>>>,
}

pub struct CfgQuenya {
    rules: HashMap<String, Vec<Vec<String>>>,
}
