//@ check-pass

#![feature(use_tree_attributes)]

mod source {
    pub struct Item;
}

use source::{#[allow(unused_imports)] Item};

fn main() {}
