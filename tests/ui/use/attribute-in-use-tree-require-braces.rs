mod inner {
    pub struct Item;
}

use crate::#[allow(unused_imports)] inner::Item; //~ ERROR expected identifier, found `#`

fn main() {
    let _: Item;
}
