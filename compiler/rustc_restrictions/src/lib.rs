// tidy-alphabetical-start
#![allow(internal_features)]
#![feature(rustdoc_internals)]
#![deny(rustc::diagnostic_outside_of_impl)]
#![deny(rustc::untranslatable_diagnostic)]
#![doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
#![doc(rust_logo)]
// tidy-alphabetical-end

mod errors;
mod impl_restriction;

use rustc_fluent_macro::fluent_messages;
use rustc_middle::query::Providers;

fluent_messages! { "../messages.ftl" }

pub fn provide(providers: &mut Providers) {
    impl_restriction::provide(providers);
}
