use crate::common::*;

use derived_deref::{Deref, DerefMut};
use godot::classes::object::ConnectFlags;
use godot::classes::tween::TransitionType;
use godot::classes::{
    ColorRect, Engine, RichTextLabel, ShaderMaterial, Theme, Tween,
};
use godot::meta::AsArg;
use godot::prelude::*;

pub fn disconnect_signal<N, SN>(node: &mut Gd<N>, signal: SN)
where
    N: Inherits<Node>,
    SN: AsArg<StringName> + Copy,
{
    let node = node.upcast_mut::<Node>();

    node.get_signal_connection_list(signal)
        .iter_shared()
        .for_each(|dict| {
            // let signal = dict.get("signal").unwrap();
            let callable = dict.get("callable").unwrap();

            node.disconnect(signal, &callable.to());
        })
}

/// Convenience function to fade opacity shaders on/off
pub fn fade_black<N>(black: &Gd<N>, visible: bool, tween_time: f64)
where
    N: GodotClass + Inherits<ColorRect>,
{
    fade_black_f64(black, visible as u8 as f64, tween_time)
}

pub fn fade_black_f64<N>(black: &Gd<N>, visible: f64, tween_time: f64)
where
    N: GodotClass + Inherits<ColorRect>,
{
    let material = black
        .upcast_ref()
        .get_material()
        .unwrap()
        .cast::<ShaderMaterial>();
    let material_id = material.instance_id();

    let callable = Callable::from_local_fn("set_shader_value", move |args| {
        let mut material = Gd::<ShaderMaterial>::from_instance_id(material_id);
        material.set_shader_parameter("opacity", &args[0]);

        Ok(Variant::nil())
    });

    let start_value = material.get_shader_parameter("opacity");

    tween_method(
        callable,
        start_value,
        visible.to_variant(),
        tween_time,
        TransitionType::QUAD,
    )
    .unwrap();
}

pub use crate::connect;
/// Macro to connect stuff without using the annoying 2-line
/// `let callable = xxxxx` syntax.
///
/// Usage:
/// ```
/// connect! {
///     node_to_connect_to,       "signal_name",
///     node_containing_callable, "callable_name";
///     // ... repeat as many as you like
/// }
/// ```
#[macro_export]
macro_rules! connect {
    ($($con_node:expr,$signal:expr=>$cal_node:expr,$cal_name:expr);* $(;)?) => {
        $({
            let callable = $cal_node.callable($cal_name);
            $con_node.connect($signal, &callable);
        })*
    };
}

/// Recursively get all children of a node that are of a certain type.
pub fn children_of_type<T, Par>(parent: &Gd<Par>) -> Vec<Gd<T>>
where
    Par: Inherits<Node>,
    T: Inherits<Node>,
{
    let mut res = vec![];
    let parent = parent.upcast_ref();

    for node in parent.get_children().iter_shared() {
        // if matches, push to res
        if let Ok(node) = node.clone().try_cast::<T>() {
            res.push(node);
        }

        // push children that match as well
        res.extend(children_of_type::<T, _>(&node));
    }

    res
}

pub fn connect_deferred<T>(node: &mut Gd<T>, signal: &str, callable: Callable)
where
    T: Inherits<Object>,
{
    node.upcast_mut()
        .connect_ex(signal, &callable)
        .flags(ConnectFlags::DEFERRED.ord() as u32)
        .done();
}

/// Returns the singleton instance of `PartyCB`.
/// So common that I might as well abbreviate it. :P
pub fn pcb() -> Gd<PartyCB> {
    PartyCB::singleton()
}

/// Returns the singleton instance `StatsInterface`.
/// So common that I might as well abbreviate it. :P
pub fn si() -> Gd<StatsInterface> {
    StatsInterface::singleton()
}

#[derive(Deref, DerefMut)]
/// Wrapper around `Gd<T>` so I can implement external traits on godot stuff
pub struct GdW<T: GodotClass>(pub Gd<T>);

/// takes a bbcode string and prepends or removes it from the label text
pub fn bbcode_toggle(mut node: Gd<RichTextLabel>, bbcode: &str, active: bool) {
    let old_text = node.get_text().to_string();
    let new_text = prefix_mod(&old_text, bbcode, active);

    node.set_text(&new_text);
}

/// adds `prefix` to the start of `target` if `active` is true...
/// otherwise removes the first `prefix.len()` characters
///
/// panics if `target` is shorter than `prefix`.
/// you also need to make sure you don't call it with `false`
/// if the prefix isn't already there. be careful with this function...
pub fn prefix_mod(target: &str, prefix: &str, active: bool) -> String {
    if active {
        format!("{}{}", prefix, target)
    } else {
        let st: String = target.into();
        st[prefix.len()..].to_owned()
    }
}

pub fn tween_method<V>(
    callable: Callable,
    start_value: V,
    end_value: V,
    time: f64,
    trans: TransitionType,
) -> Result<Gd<Tween>, ()>
where
    V: ToGodot,
{
    let res: Option<_> = try {
        let mut tween = godot_tree().create_tween()?;

        tween
            .tween_method(
                &callable,
                &start_value.to_variant(),
                &end_value.to_variant(),
                time,
            )?
            .set_trans(trans);

        tween
    };

    res.ok_or(())
}

/// shorthand to do some tweeneroonies :3
/// `time` is in seconds
pub fn tween<NP, V, N>(
    node: &mut Gd<N>,
    property: NP,
    start_value: Option<V>,
    end_value: V,
    time: f64,
    trans: TransitionType,
) -> Result<Gd<Tween>, ()>
where
    NP: AsArg<NodePath>,
    V: ToGodot,
    N: Inherits<Node> + Inherits<Object>,
{
    let res: Option<_> = try {
        let mut tween = node.upcast_mut::<Node>().create_tween()?;

        let mut property = tween
            .tween_property(&*node, property, &end_value.to_variant(), time)?
            .set_trans(trans)?;

        if let Some(start_value) = start_value {
            property.from(&start_value.to_variant())?;
        }

        tween
    };

    res.ok_or(())
}

pub fn default_theme() -> Gd<Theme> {
    load("res://themes/theme_deft.tres")
}

pub fn godot_tree() -> Gd<SceneTree> {
    Engine::singleton().get_main_loop().unwrap().cast()
}

pub fn current_scene() -> Gd<Node> {
    godot_tree().get_current_scene().unwrap()
}

pub use crate::change_scene;
#[macro_export]
macro_rules! change_scene {
    ($scene:expr) => {
        godot_tree().change_scene_to_file(concat!(
            "res://scenes/",
            $scene,
            ".tscn"
        ))
    };
}

pub fn mark_input_handled<T>(node: &Gd<T>)
where
    T: Inherits<Node>,
{
    node.upcast_ref::<Node>()
        .get_viewport()
        .unwrap()
        .set_input_as_handled();
}
