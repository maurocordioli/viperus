///ViperusValue encaspule data values of type String,i32 and bool
///
///implements bidirectional conversion to respective  values via Into<T> and From<T>
/// # Example
/// ```
/// use viperus::ViperusValue;
/// let x:i32=ViperusValue::I32(42).into();
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub enum ViperusValue {
    Empty,
    Str(String),
    I32(i32),
    BOOL(bool),
}

impl Into<bool> for &ViperusValue {
    fn into(self) -> bool {
        if let ViperusValue::BOOL(i) = self {
            *i
        } else {
            panic!("not a bool")
        }
    }
}

impl Into<bool> for ViperusValue {
    fn into(self) -> bool {
        if let ViperusValue::BOOL(i) = self {
            i
        } else {
            panic!("not a bool")
        }
    }
}

impl From<bool> for ViperusValue {
    fn from(src: bool) -> ViperusValue {
        ViperusValue::BOOL(src)
    }
}

impl From<i32> for ViperusValue {
    fn from(src: i32) -> ViperusValue {
        ViperusValue::I32(src)
    }
}

impl Into<i32> for &ViperusValue {
    fn into(self) -> i32 {
        if let ViperusValue::I32(i) = self {
            *i
        } else {
            panic!("not an i32")
        }
    }
}

impl Into<i32> for ViperusValue {
    fn into(self) -> i32 {
        if let ViperusValue::I32(i) = self {
            i
        } else {
            panic!("nnot an i32")
        }
    }
}

impl From<String> for ViperusValue {
    fn from(src: String) -> ViperusValue {
        ViperusValue::Str(src)
    }
}

impl<'a> From<&'a String> for ViperusValue {
    fn from(src: &'a String) -> ViperusValue {
        ViperusValue::Str(src.clone())
    }
}

impl From<&str> for ViperusValue {
    fn from(src: &str) -> ViperusValue {
        ViperusValue::Str(src.to_owned())
    }
}

impl<'a> Into<&'a str> for &'a ViperusValue {
    fn into(self) -> &'a str {
        if let ViperusValue::Str(i) = self {
            i
        } else {
            panic!("not an str")
        }
    }
}

impl<'a> Into<String> for &'a ViperusValue {
    fn into(self) -> String {
        if let ViperusValue::Str(i) = self {
            i.clone()
        } else {
            panic!("not an str")
        }
    }
}

impl Into<String> for ViperusValue {
    fn into(self) -> String {
        if let ViperusValue::Str(i) = self {
            i
        } else {
            panic!("not a string")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[should_panic]
    fn invalid_cast_mv2bool() {
        init();

        let mv = ViperusValue::Empty;
        let b: bool = mv.into();
      }

    #[test]
    #[should_panic]
    fn invalid_cast_refmv2bool() {
        init();

        let mv = &ViperusValue::Empty;
        let b: bool = mv.into();
    }

    #[test]
    #[should_panic]
    fn invalid_cast_mv2i32() {
        init();

        let mv = &ViperusValue::Empty;
        let _b: i32 = mv.into();
    }
    #[test]
    fn valid_cast_mv2bool() {
        init();

        let mv = ViperusValue::BOOL(true);
        let b: bool = mv.into();
        assert!(b);
    }

    #[test]
    fn valid_cast_str2mv() {
        init();

        let mv = ViperusValue::from("hello world!");
        if let ViperusValue::Str(s) = mv {
            assert_eq!(s, "hello world!");
        } else {
            panic!("something very wrong");
        }

        let refmv = ViperusValue::from(&("hello world!".to_owned()));
        if let ViperusValue::Str(s) = refmv {
            assert_eq!(s, "hello world!");
        } else {
            panic!("something very wrong");
        }


  
    }

    #[test]
    #[should_panic]
    fn invalid_cast_mv2string() {
        init();

        let mv = &ViperusValue::Empty;
        let _b: String = mv.into();
    }
  
}
