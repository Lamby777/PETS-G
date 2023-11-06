//!
//! This module is for the data structures used
//! for the actual dialogue text, choices, etc.
//!

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct DialogueNode {
    // TODO: static string
    id: String,

    // TODO replace with &'a str
    speaker: String,
    vox: String,

    text: String,
    options: Option<Vec<DialogueChoice>>,
}

/// Possible outcomes of picking a dialogue option.
///
/// "Yeah, this new name's WAY less confusing... right?"
/// - Devon, 2037
#[derive(Serialize, Deserialize)]
pub enum DialogueAction {
    /// Leads to another node
    /// (simple `A -> (B|C)` dialogue)
    NextNode(DialogueNode),

    /// Leads to running a function
    /// (fancy stuff like shops)
    /// TODO static string
    Action(String),

    /// End the tree with this option and (usually)
    /// return control to the user ("goodbye")
    End,
}

#[derive(Serialize, Deserialize)]
pub struct DialogueChoice {
    /// the text saying what the choice is
    label: String,

    /// whether the choice is selectable or grayed out
    /// TODO: maybe should be separate from this struct?
    available: bool,

    /// the action this choice leads to
    action: DialogueAction,
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO test for options

    #[test]
    fn dnode_fields() {
        let dnode = DialogueNode {
            id: "test_interaction".to_string(),
            speaker: "Cherry".to_string(),
            vox: "Mira".to_string(),
            text: "This dialogue is for testing purposes".to_string(),
            options: None,
        };
    }
}
