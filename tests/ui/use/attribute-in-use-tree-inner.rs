mod inner {
    pub struct Item;
}

use crate::{#![allow(unused_imports)] inner::Item}; //~ ERROR an inner attribute is not permitted in this context

fn main() {
    let _: Item;
}
