//!
//! Dialogue system for the game's menus.
//!

pub mod autoload;
pub mod dbox;

/// Possible outcomes of picking a dialogue option.
///
/// Damn, we're really calling it this?
/// Enum? Option? Result? What do you think this is, Rust?
/// Oh, wait... it IS Rust...
///
/// - Devon, 2037
enum DialogueOptionResult {
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

pub struct DialogueOption {
    label: String,
    available: bool,
    leads_to: DialogueOptionResult,
}

pub struct DialogueNode {
    // TODO replace with &'a str
    speaker: String,
    vox: String,

    text: String,
    options: Vec<DialogueOption>,
}
