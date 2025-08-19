use std::sync::LazyLock;

// use crate::common::*;

use string_interner::{DefaultBackend, StringInterner};

pub static mut INTERNER: LazyLock<StringInterner<DefaultBackend>> =
    LazyLock::new(StringInterner::default);
