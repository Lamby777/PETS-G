//!
//! This module is for the data structures used
//! for the actual dialogue text, choices, etc.
//!

/// Possible outcomes of picking a dialogue option.
///
/// "Yeah, this new name's WAY less confusing... right?"
/// - Devon, 2037
pub enum DialogueAction {
    /// Leads to another node
    /// (simple `A -> (B|C)` dialogue)
    NextNode(DialogueNode),

    /// Leads to running a function pointer
    /// (fancy stuff like shops)
    Action(fn()),

    /// End the tree with this option and (usually)
    /// return control to the user ("goodbye")
    End,
}

pub struct DialogueChoice {
    label: String,
    available: bool,
    leads_to: DialogueAction,
}

pub struct DialogueNode {
    // TODO replace with &'a str
    speaker: String,
    vox: String,

    text: String,
    options: Vec<DialogueChoice>,
}
