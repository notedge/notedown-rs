use super::*;



impl Add for Value {
    type Output = Result<Self>;

    fn add(self, other: Self) -> Self::Output {
        let out = match (self, other) {
            (Self::String(lhs), Self::String(rhs)) => {
                lhs.push(rhs)
            },
            _ => return Err(NoteError::)
        };
        return Ok(out)
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}