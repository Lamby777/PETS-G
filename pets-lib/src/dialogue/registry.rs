//!
//! This module is for loading dialogue trees using serde
//!

use crate::prelude::*;

// TODO: &str keys
/// Map of all the named dialogue nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueMap(HashMap<String, DialogueNode>);

impl DialogueMap {
    /// Load a dialogue map from a file
    pub fn load_from_file(_path: &str) -> Result<Self> {
        // let map = serde_json::
        Ok(Self(HashMap::new()))
    }
}
