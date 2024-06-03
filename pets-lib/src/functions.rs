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
pub fn callv_global(func_id: &str, args: VariantArray) -> GReturn {
    let funcs = FUNCTIONS;
    let func = funcs.get(func_id);
    let func = unwrap_fmt!(func, "no function named {}", func_id);

    println!("{}", args.front().unwrap().to::<String>());

    let res = func.callv(args);
    println!("5");
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

fn debug_item(args: GArgs) -> GReturn {
    for arg in args {
        println!("giving a {}", arg.to::<String>());

        let item_id = arg.to::<String>();

        // why tf do i have to do this?
        let item = ITEM_REGISTRY
            .get()
            .unwrap()
            .iter()
            .find(|i| i.id == item_id);

        let item = ribbons::unwrap_fmt!(item, "no item with id {}", item_id);

        give_item(item.clone());
    }

    end_interaction();
    start_ix("Debug Menu >> After Item");

    Ok(Variant::nil())
}
