// use crate::prelude::*;
// use godot::engine::{Sprite2D, Texture2D};
// use godot::prelude::*;
//
// fn set_ethan_bed_color(color: String) {
//     let texture = load::<Texture2D>(format!(
//         "res://assets/textures/builds/furniture/beds/bed_{color}.png"
//     ));
//
//     si().bind_mut().save.bed_color = color;
//
//     let mut bed = World::room().get_node_as::<Sprite2D>("%EthanBed/Sprite2D");
//     bed.callv("set_texture".into(), varray![texture]);
//
//     end_ix();
// }
//
// fn debug_battle() {
//     World::start_battle(&EnemyID::A_NONNY_MOUSE);
// }
//
// fn debug_item(item_id: String, quantity: u32) {
//     Inventory::get().borrow_mut().give_item(item_id, quantity);
// }
