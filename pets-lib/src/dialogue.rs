/*
    Dialogue system for the game.
*/

#![allow(dead_code)]

enum DialogueOptionResult {
    // can either load to another node
    NextNode(DialogueNode),

    // or could also be a function pointer
    Action(fn()),
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
