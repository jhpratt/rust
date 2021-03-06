#![crate_name = "foo"]

pub trait Bar<T, U> {}

// @has 'foo/struct.Foo1.html'
pub struct Foo1;
// @count - '//*[@id="trait-implementations-list"]//*[@class="impl has-srclink"]' 1
// @has - '//*[@class="impl has-srclink"]' "impl Bar<Foo1, &'static Foo1> for Foo1"
impl Bar<Foo1, &'static Foo1> for Foo1 {}

// @has 'foo/struct.Foo2.html'
pub struct Foo2;
// @count - '//*[@id="trait-implementations-list"]//*[@class="impl has-srclink"]' 1
// @has - '//*[@class="impl has-srclink"]' "impl Bar<&'static Foo2, Foo2> for u8"
impl Bar<&'static Foo2, Foo2> for u8 {}
