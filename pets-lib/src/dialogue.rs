/*
    Dialogue system for the game.
*/

#![allow(dead_code)]

use godot::engine::Font;
use godot::prelude::*;

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

struct DialogueOption {
    label: String,
    available: bool,
    leads_to: DialogueOptionResult,
}

struct DialogueNode {
    text: Vec<StringSegment>,

    speaker: String,
    vox: String,

    options: Vec<DialogueOption>,
}

/// Part of a string, meant to be joined
/// with other parts to have partially
/// formatted sections in a single message
struct StringSegment {
    text: String,
    color: Color,
    font: Font,

    /// stuff like in Omori/Celeste where the
    /// dialogue text shakes or moves around.
    // movement: DialogueTextMovement,

    /// if Some, this will override the default
    /// voice if inside a dialogue tree.
    // TODO maybe be a newtype? is this level of
    // control over voicelines even necessary?
    vox: Option<String>,
}
