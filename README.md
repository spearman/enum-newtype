# `enum-newtype`

> `enum_newtype!` macro for implementing tagged "newtype" enums of the form
> `enum Myenum { Foo(Foo), Bar(Bar), Baz(Baz) }`

## Usage

The macro `enum_newtype!` expands into an enum definition and implementations of
`From` and `TryFrom` for each corresponding variant.

*Example*:

```
use std::convert::TryFrom;
#[derive(PartialEq)]
struct Foo {
  pub x : u8
}
#[derive(PartialEq)]
struct Bar {
  pub s : String
}
enum_newtype!{
  #[derive(PartialEq)]
  enum Myenum {
    Foo (Foo),
    Bar (Bar)
  }
}
let a = Myenum::from (Foo { x: 5 });
let b = Myenum::from (Bar { s: "abc".to_string() });
let c = Myenum::from (Foo { x: 5 });
assert!(a != b);
assert!(a == c);
assert!(Foo::try_from (a).is_ok());
assert!(Foo::try_from (b).is_err());
assert!(Foo::try_from (c).is_ok());
```
