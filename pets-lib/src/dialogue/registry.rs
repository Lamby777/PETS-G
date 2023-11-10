//!
//! This module is for loading dialogue trees using serde
//!

use crate::prelude::*;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueMap {
    // TODO: &str keys
    /// Map of all the named dialogue nodes
    pub nodes: HashMap<String, DialogueNode>,
}

impl DialogueMap {
    /// Load a dialogue map from a file
    pub fn load_from_file(path: &str) -> Result<Self> {
        //
        // let map = serde_json::
        Ok(Self {
            nodes: HashMap::new(),
        })
    }
}
