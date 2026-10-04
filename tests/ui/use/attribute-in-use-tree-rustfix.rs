//@ check-pass

#![feature(use_tree_attributes)]
#![allow(dead_code)]
#![warn(unused_imports)]

mod unused {
    pub struct Allowed;
    pub struct Unannotated;
}

use crate::{
    #[allow(unused_imports)]
    unused::Allowed,
    unused::Unannotated,
    //~^ WARN unused import: `unused::Unannotated`
};

fn main() {}
