//!
//! Area that allows the player to start
//! an interaction when within range
//!

use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;

use crate::prelude::*;
use crate::world::playercb::PlayerCB;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct InteractionZone {
    #[base]
    node: Base<Area2D>,

    #[export]
    name: GString,
}

#[godot_api]
impl InteractionZone {
    #[func]
    pub fn interact(&self) {
        use dialogical::Metaline::*;
        use dialogical::Speaker::*;

        let ix_list = ix_list();
        let ix = ix_list.get("Rodrick Sign #1").unwrap();
        let page = ix.pages.get(0).unwrap();
        let spk = page.metadata.speaker.clone();

        let spk = match spk {
            PageOnly(v) | Permanent(v) => v,
            NoChange => unreachable!(),
        };

        let spk = match spk {
            Named(v) => v,
            Narrator => "".to_string(),
            Unknown => "".to_string(),
        };

        show_dialog!(spk, "{}", page.content.clone());
    }

    #[func]
    fn on_entered(&mut self, _body: Gd<PlayerCB>) {
        let mut im = InteractionManager::singleton();
        im.bind_mut().register_zone(self.node.clone().cast());
    }

    #[func]
    fn on_exited(&mut self, _body: Gd<PlayerCB>) {
        let mut im = InteractionManager::singleton();
        im.bind_mut().unregister_zone(self.node.clone().cast());
    }
}

#[godot_api]
impl IArea2D for InteractionZone {
    fn init(node: Base<Area2D>) -> Self {
        Self {
            node,
            name: "".into(),
        }
    }

    fn ready(&mut self) {
        let node = &mut self.node;

        let enter_fn = Callable::from_object_method(node, "on_entered");
        let exit_fn = Callable::from_object_method(node, "on_exited");
        node.connect("body_entered".into(), enter_fn);
        node.connect("body_exited".into(), exit_fn);
    }
}
