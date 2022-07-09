#![doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]

mod impl_restriction;

use rustc_middle::ty::query::Providers;

pub fn provide(providers: &mut Providers) {
    impl_restriction::provide(providers);
}
