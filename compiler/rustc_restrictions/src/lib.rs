#![doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
#![feature(box_patterns)]

mod impl_restriction;
mod mut_restriction;

use rustc_middle::ty::query::Providers;

pub fn provide(providers: &mut Providers) {
    impl_restriction::provide(providers);
    mut_restriction::provide(providers);
}
