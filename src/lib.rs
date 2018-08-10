#![feature(try_from)]

#[macro_export]
macro_rules! enum_newtype {
  (
    $(#[$attr:meta])*
    enum $enum:ident {
      $($variant:ident ($type:ty)),+
    }
  ) => {
    $(#[$attr])*
    enum $enum {
      $($variant ($type)),+
    }

    $(
    impl From <$type> for $enum {
      fn from (payload : $type) -> Self {
        $enum::$variant (payload)
      }
    }
    )+

    $(
    impl ::std::convert::TryFrom <$enum> for $type {
      type Error = $enum;
      fn try_from (value : $enum) -> Result <Self, $enum> {
        match value {
          $enum::$variant (payload) => Ok (payload),
          _ => Err (value)
        }
      }
    }
    )+
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_unit() {
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
  }
}
