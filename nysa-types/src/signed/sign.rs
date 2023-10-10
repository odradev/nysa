use core::{
    fmt::{self, Write},
    ops,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum Sign {
    Negative = -1,
    Positive = 1,
}

impl ops::Mul for Sign {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        if self == rhs {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

impl ops::Neg for Sign {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}

impl ops::Not for Sign {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self, f.sign_plus()) {
            (Self::Positive, false) => Ok(()),
            _ => f.write_char(self.as_char()),
        }
    }
}

impl Sign {
    #[inline]
    pub const fn as_char(&self) -> char {
        match self {
            Self::Positive => '+',
            Self::Negative => '-',
        }
    }
}
