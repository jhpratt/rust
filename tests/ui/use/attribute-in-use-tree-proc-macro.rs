//@ proc-macro: attribute-in-use-tree-macro.rs

extern crate attribute_in_use_tree_macro;

mod inner {
    pub struct Item;
}

use attribute_in_use_tree_macro::marker_attr;

use crate::{
    #[marker_attr] inner::Item, //~ ERROR only cfg, lint, stability, and doc attributes are allowed on use tree entries
};

fn main() {
    let _: Item;
}
