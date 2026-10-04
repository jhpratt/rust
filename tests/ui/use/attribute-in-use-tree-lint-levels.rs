#![feature(use_tree_attributes)]
#![deny(unused_imports)]
#![allow(dead_code)]

mod unused {
    pub struct Allowed;
    pub struct Expected;
    pub struct Denied;
}

use crate::{
    #[allow(unused_imports)]
    unused::Allowed,
    #[expect(unused_imports)]
    unused::Expected,
    unused::Denied,
    //~^ ERROR unused import: `unused::Denied`
};

fn main() {}
