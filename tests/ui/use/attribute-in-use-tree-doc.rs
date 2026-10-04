//@ check-pass

#![feature(use_tree_attributes)]
#![allow(unused_imports)]

mod inner {
    pub struct Item;
}

pub use crate::{
    #[doc(hidden)]
    inner::Item as Hidden,
    #[doc(inline)]
    inner::Item as Inline,
    /// Documentation for this re-export.
    #[doc(no_inline)]
    inner::Item as NoInline,
};

fn main() {}
