use super::*;


macro_rules! into_string {
    ($T:ty) => {
        impl From<$T> for PrimitiveValue {
            fn from(s: $T) -> Self {
                Self::String(s.to_string())
            }
        }
    };
    ($($T:ty),+ $(,)?) => {
        $(into_string!($T);)+
    };
}

macro_rules! into_integer {
    ($T:ty) => {
        impl From<$T> for PrimitiveValue {
            fn from(s: $T) -> Self {
                Self::Integer(BigInt::from(v))
            }
        }
    };
    ($($T:ty),+ $(,)?) => {
        $(into_string!($T);)+
    };
}

impl From<()> for PrimitiveValue {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<bool> for PrimitiveValue {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl From<f32> for PrimitiveValue {
    fn from(v: f32) -> Self {
        Self::from(v as f64)
    }
}

impl From<f64> for PrimitiveValue {
    fn from(v: f64) -> Self {
        unsafe {
            Self::Decimal(transmute::<f64, [u8; 8]>(v))
        }
    }
}

into_integer![u8, u16,u32,u64,u128,usize, BigUint];
into_integer![i8, i16,i32,i64,i128,isize];
into_string![char, &str, String];

#[test]
fn test() {
    println!("{:?}", PrimitiveValue::from(""))
}
