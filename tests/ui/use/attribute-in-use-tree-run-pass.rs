//@ check-pass

#![deny(unused_imports)]
#![allow(dead_code)]

mod available {
    pub struct Item;
}

mod unused {
    pub struct Item;
}

use crate::{
    #[cfg(true)] available::{Item},
    #[cfg(false)] missing::{Item as Missing},
    #[allow(unused_imports)] unused::Item as Unused,
    #[expect(unused_imports)] unused::Item as Expected,
    #[cfg_attr(false, cfg(false))] available::Item as AlsoAvailable,
};

fn main() {
    let _: Item;
    let _: AlsoAvailable;
}
