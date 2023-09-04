/*
    Dialogue system for the game.
*/

#![allow(dead_code)]

/// Possible outcomes of picking a dialogue option.
///
/// Damn, we're really calling it this?
/// Enum? Option? Result? What do you think this is, Rust?
/// Oh, wait... it IS Rust...
///
/// - Devon, 2037
enum DialogueOptionResult {
    // can either load to another node
    // (simple A -> (B or C) type of dialogue)
    NextNode(DialogueNode),

    // or could also be a function pointer
    // (fancy stuff like shops)
    Action(fn()),

    // or just end the tree with that option
    // and return control to the user ("goodbye")
    End,
}

struct DialogueOption {
    label: String,
    leads_to: DialogueOptionResult,
}

struct DialogueNode {
    text: String,
    options: Vec<DialogueOption>,
}

fn dialogue_yes_no() {
    unimplemented!();
}
