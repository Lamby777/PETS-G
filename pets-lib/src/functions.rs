use std::cell::LazyCell;

use crate::prelude::*;
use godot::prelude::*;

fn end_interaction() {
    DialogBox::singleton().bind_mut().end_interaction();
}

macro_rules! add_callables {
    ($table:expr; { $($fn_name:ident),* $(,)? }) => {
        $(
            let name = stringify!($fn_name);
            let callable = Callable::from_fn(name, $fn_name);
            $table.insert(name.to_string(), callable);
        )*
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
    add_callables!(table; {
        debug_battle,
        debug_item,
    });

    table
});

fn debug_battle(_args: GArgs) -> GReturn {
    end_interaction();

    World::start_battle(EnemyID::A_NONNY_MOUSE.into());
    Ok(Variant::nil())
}

fn debug_item(_args: GArgs) -> GReturn {
    end_interaction();

    let mut si = si();
    let inv = &mut si.bind_mut().save.inventory;

    inv.push(TRUSTY_RUSTY.clone());
    godot_print!("Inventory: {:?}", inv);

    Ok(Variant::nil())
}
