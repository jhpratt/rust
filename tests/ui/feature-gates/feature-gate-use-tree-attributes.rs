//@ check-fail

mod source {
    pub struct Item;
}

use source::{#[allow(unused_imports)] Item};
//~^ ERROR attributes on use tree entries are unstable

fn main() {}
