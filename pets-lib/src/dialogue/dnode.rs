//!
//! This module is for the data structures used
//! for the actual dialogue text, choices, etc.
//!

use crate::prelude::*;

/// `Ok` if picked, `Err` if option was grayed out
pub type DialogueChoiceResult<T> = Result<T, ()>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DialogueNode {
    // TODO: static string
    id: String,

    // TODO replace with &'a str
    speaker: String,
    vox: String,

    text: String,
    options: Option<Vec<DialogueChoice>>,
}

impl DialogueNode {
    pub fn option(&self, index: usize) -> DialogueChoiceResult<&DialogueChoice> {
        // if the index is out of range or the result has
        // availabe: false, return err
        let opts = self.options.as_ref().ok_or(())?;
        let opt = opts.get(index).ok_or(())?;

        if opt.available {
            Ok(opt)
        } else {
            Err(())
        }
    }
}

/// Possible outcomes of picking a dialogue option.
///
/// "Yeah, this new name's WAY less confusing... right?"
/// - Devon, 2037
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    fn dnode_pick_unavailable_option() {
        let op1 = DialogueChoice {
            label: "Test option 1".to_string(),
            available: true,
            action: DialogueAction::End,
        };
        let op2 = DialogueChoice {
            label: "Test option 2".to_string(),
            available: false,
            action: DialogueAction::End,
        };

        let dnode = DialogueNode {
            id: "test_interaction".to_string(),
            speaker: "Cherry".to_string(),
            vox: "Mira".to_string(),
            text: "This dialogue is for testing purposes".to_string(),
            options: Some(vec![op1.clone(), op2.clone()]),
        };

        assert_eq!(*dnode.option(0).unwrap(), op1);
        assert!(dnode.option(1).is_err());
    }
}
