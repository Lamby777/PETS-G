use std::cell::LazyCell;

use crate::prelude::*;
use godot::prelude::*;

macro_rules! add_callables {
    ($table:expr; $($fn_name:ident),* $(,)?) => {
        $(
            let name = stringify!($fn_name);
            let callable = Callable::from_fn(name, $fn_name);
            $table.insert(name.to_string(), callable);
        ),*
    };
}

/// Call a function registered in the global function table.
/// To pass args, use `callv_global`.
pub fn call_global(id: &str) -> GReturn {
    callv_global(id, VariantArray::new())
}

/// Call a function registered in the global function table.
pub fn callv_global(id: &str, args: VariantArray) -> GReturn {
    let funcs = FUNCTIONS;
    let func = funcs.get(id);
    let func = unwrap_fmt!(func, "no function named {}", id);

    let res = func.callv(args);
    Ok(res)
}

const FUNCTIONS: LazyCell<FnTable> = LazyCell::new(|| {
    let mut table = HashMap::new();
    add_callables!(table; debug_battle);

    table
});

fn debug_battle(_args: GArgs) -> GReturn {
    // end interaction
    let mut dbox = DBoxInterface::singleton().bind().dbox();
    dbox.bind_mut().end_interaction();

    // start the battle
    let dbg_eid = EnemyID::A_NONNY_MOUSE;
    World::start_battle(dbg_eid.into());

    Ok(Variant::nil())
}

// #[func]
// pub fn debug_llm() {
//     crate::llm::llm_generate();
// }
