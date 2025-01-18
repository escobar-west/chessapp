use crate::errors::ParseFenError;
use std::{
    ops::{BitAndAssign, BitOrAssign, Not},
    str::FromStr,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Castle {
    Null,
    K,
    Q,
    KQ,
    k,
    Kk,
    Qk,
    KQk,
    q,
    Kq,
    Qq,
    KQq,
    kq,
    Kkq,
    Qkq,
    KQkq,
}

impl BitAndAssign for Castle {
    fn bitand_assign(&mut self, rhs: Self) {
        let ptr = self as *mut Self as *mut u8;
        // Safety: ptr points to valid u8 and does not produce invalid variant
        unsafe {
            *ptr &= rhs as u8;
        }
    }
}
impl BitOrAssign for Castle {
    fn bitor_assign(&mut self, rhs: Self) {
        let ptr = self as *mut Self as *mut u8;
        // Safety: ptr points to valid u8 and does not produce invalid variant
        unsafe {
            *ptr |= rhs as u8;
        }
    }
}

impl Not for Castle {
    type Output = Self;
    fn not(self) -> Self::Output {
        let output = Self::KQkq as u8 & !(self as u8);
        // Safety: KQkq & any value is valid variant
        unsafe { std::mem::transmute::<u8, Self>(output) }
    }
}

impl ToString for Castle {
    fn to_string(&self) -> String {
        let output = match self {
            Castle::Null => "-",
            Castle::K => "K",
            Castle::Q => "Q",
            Castle::KQ => "KQ",
            Castle::k => "k",
            Castle::Kk => "Kk",
            Castle::Qk => "Qk",
            Castle::KQk => "KQk",
            Castle::q => "q",
            Castle::Kq => "Kq",
            Castle::Qq => "Qq",
            Castle::KQq => "KQq",
            Castle::kq => "kq",
            Castle::Kkq => "Kkq",
            Castle::Qkq => "Qkq",
            Castle::KQkq => "KQkq",
        };
        output.into()
    }
}

impl FromStr for Castle {
    type Err = ParseFenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Castle::Null),
            "K" => Ok(Castle::K),
            "Q" => Ok(Castle::Q),
            "KQ" => Ok(Castle::KQ),
            "k" => Ok(Castle::k),
            "Kk" => Ok(Castle::Kk),
            "Qk" => Ok(Castle::Qk),
            "KQk" => Ok(Castle::KQk),
            "q" => Ok(Castle::q),
            "Kq" => Ok(Castle::Kq),
            "Qq" => Ok(Castle::Qq),
            "KQq" => Ok(Castle::KQq),
            "kq" => Ok(Castle::kq),
            "Kkq" => Ok(Castle::Kkq),
            "Qkq" => Ok(Castle::Qkq),
            "KQkq" => Ok(Castle::KQkq),
            _ => Err(ParseFenError::InvalidString(s.into())),
        }
    }
}
