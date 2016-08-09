use super::*;

impl Add for Value {
    type Output = Result<Self>;

    /// a + b
    fn add(self, other: Self) -> Self::Output {
        let msg = format!("Can not apply `+` on lhs: {}, rhs: {}", self.get_type_name(), other.get_type_name());
        let type_mismatch = Err(NoteError::type_mismatch(msg));

        let out = match (self, other) {
            (Self::String(lhs), Self::String(rhs)) => Self::String(lhs + &rhs),
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs + &rhs),
            (Self::Integer(lhs), Self::Decimal(rhs)) | (Self::Decimal(rhs), Self::Integer(lhs)) => match lhs.to_f64() {
                Some(s) => Self::Decimal(s + rhs),
                None => return fail_int2dec(lhs),
            },
            (Self::Decimal(lhs), Self::Decimal(rhs)) => Self::Decimal(lhs + rhs),
            _ => return type_mismatch,
        };
        return Ok(out);
    }
}

impl Shr for Value {
    type Output = Result<Self>;

    /// a ++ b
    fn shr(self, other: Self) -> Self::Output {
        let msg = format!("Can not apply `++` on lhs: {}, rhs: {}", self.get_type_name(), other.get_type_name());
        let type_mismatch = Err(NoteError::type_mismatch(msg));
        let out = match (self, other) {
            (Self::String(lhs), rhs) => Self::string_join(lhs, rhs)?,
            _ => return type_mismatch,
        };
        return Ok(out);
    }
}

impl Value {
    pub fn string_join(lhs: String, other: Value) -> Result<Self> {
        let msg = format!("Can not apply `++` on lhs: `String`, rhs: {}", other.get_type_name());
        let type_mismatch = Err(NoteError::type_mismatch(msg));
        let out = match other {
            Value::Null => lhs,
            Value::Boolean(v) => {
                format!("{}{}", lhs, v)
            }
            Value::Integer(v) => {
                format!("{}{}", lhs, v)
            }
            Value::Decimal(v) => {
                format!("{}{}", lhs, v)
            }
            Value::String(v) => {
                format!("{}{}", lhs, v)
            }
            Value::Set(_) => {
                return type_mismatch;
            }
            Value::Array(_) => {
                return type_mismatch;
            }
            Value::Object(_) => {
                return type_mismatch;
            }
        };
        Ok(Value::String(out))
    }
}

impl Sub for Value {
    type Output = Result<Self>;
    /// a- b
    fn sub(self, other: Self) -> Self::Output {
        let msg = format!("Can not apply `-` on lhs: {}, rhs: {}", self.get_type_name(), other.get_type_name());
        let type_mismatch = Err(NoteError::type_mismatch(msg));

        let out = match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs - rhs),
            (Self::Integer(lhs), Self::Decimal(rhs)) | (Self::Decimal(rhs), Self::Integer(lhs)) => match lhs.to_f64() {
                Some(s) => Self::Decimal(s - rhs),
                None => return fail_int2dec(lhs),
            },
            (Self::Decimal(lhs), Self::Decimal(rhs)) => Self::Decimal(lhs - rhs),
            _ => return type_mismatch,
        };
        return Ok(out);
    }
}
