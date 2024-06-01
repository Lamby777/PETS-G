use std::cell::LazyCell;

use crate::prelude::*;
use godot::prelude::*;

fn end_interaction() {
    DialogBox::try_singleton()
        .unwrap()
        .bind_mut()
        .end_interaction();
}

fn give_item(item: Item) {
    let inv = si().bind_mut().save.inventory.clone();
    inv.borrow_mut().push(item);
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
        debug_item_rusty,
    });

    table
});

fn debug_battle(_args: GArgs) -> GReturn {
    end_interaction();

    World::start_battle(EnemyID::A_NONNY_MOUSE.into());
    Ok(Variant::nil())
}

fn after_debug_item() {
    end_interaction();
    start_ix("Debug Menu >> After Item");
}

fn debug_item_rusty(_args: GArgs) -> GReturn {
    give_item(TRUSTY_RUSTY.clone());
    after_debug_item();

    Ok(Variant::nil())
}
