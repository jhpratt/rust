//@ check-pass

#![feature(use_tree_attributes)]
#![allow(unused_imports)]

use foo::{
    #[cfg(true)]
    bar,
    #[cfg(false)]
    baz,
};

// Make sure we handle reserved symbols (leading `::` is `sym::PathRoot`).
use ::foo::{
    #[cfg(false)]
    qux,
};

mod foo {
    pub(crate) mod bar {}
    pub(crate) mod baz {}
    pub(crate) mod qux {}
}

fn main() {}
