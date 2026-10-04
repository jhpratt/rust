//@ run-rustfix
//@ check-pass

#![allow(dead_code)]
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
