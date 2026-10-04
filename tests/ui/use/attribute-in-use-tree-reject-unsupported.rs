mod inner {
    pub struct Item;
}

use crate::{#[inline] inner::Item};
//~^ ERROR only cfg, lint, stability, and doc attributes are allowed on use tree entries
//~| ERROR the `inline` attribute cannot be used on use statements

fn main() {
    let _: Item;
}
