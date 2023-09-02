use godot::prelude::*;
use libdx::Result;

fn main() -> Result<()> {
    Ok(())
}

type DialogueOptions = Vec<DialogueNode>;

struct DialogueNode {
    text: String,
    options: DialogueOptions,
}

fn dialogue_yes_no() {
    unimplemented!();
}
