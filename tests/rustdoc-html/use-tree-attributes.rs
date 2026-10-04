#![crate_name = "use_tree_attributes"]

mod source {
    pub struct Hidden;
    pub struct Inline;
    pub struct NoInline;
}

//@ has use_tree_attributes/index.html
//@ has use_tree_attributes/struct.Inline.html
//@ !has use_tree_attributes/struct.Hidden.html
//@ hasraw use_tree_attributes/index.html 'pub use source::NoInline;'
//@ !hasraw use_tree_attributes/index.html 'pub use source::Inline;'
pub use source::{
    #[doc(hidden)]
    Hidden,
    #[doc(inline)]
    Inline,
    #[doc(no_inline)]
    NoInline,
};
