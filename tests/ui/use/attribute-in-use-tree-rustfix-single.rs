//@ run-rustfix
//@ check-pass

#![feature(use_tree_attributes)]
#![allow(dead_code)]
#![allow(unused_features)]
#![warn(unused_imports)]

mod source {
    pub struct Unused;
    pub struct Used;
}

use crate::source::{
    #[doc(hidden)]
    #[warn(unused_imports)]
    Unused,
    //~^ WARN unused import: `Unused`
    #[doc(inline)]
    Used,
};

fn main() {
    let _ = Used;
}
