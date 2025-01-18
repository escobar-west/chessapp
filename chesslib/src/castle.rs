use crate::{errors::ParseFenError, pieces::Color};
use std::{
    fmt::Display,
    ops::{BitAndAssign, BitOrAssign, Not},
    str::FromStr,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Castle {
    Null, // 0b0000
    K,    // 0b0001
    Q,    // 0b0010
    KQ,
    k, // 0b0100
    Kk,
    Qk,
    KQk,
    q, // 0b1000
    Kq,
    Qq,
    KQq,
    kq,
    Kkq,
    Qkq,
    KQkq, // 0b1111
}

impl Castle {
    pub fn remove_castle(&mut self, color: Color) {
        let ptr = self as *mut Self as *mut u8;
        let mask = match color {
            Color::White => Castle::kq as u8,
            Color::Black => Castle::KQ as u8,
        };
        unsafe {
            *ptr &= mask;
        }
    }

    pub fn remove_king_castle(&mut self, color: Color) {
        let ptr = self as *mut Self as *mut u8;
        let mask = match color {
            Color::White => Castle::Qkq as u8,
            Color::Black => Castle::KQq as u8,
        };
        unsafe {
            *ptr &= mask;
        }
    }

    pub fn remove_queen_castle(&mut self, color: Color) {
        let ptr = self as *mut Self as *mut u8;
        let mask = match color {
            Color::White => Castle::Kkq as u8,
            Color::Black => Castle::KQk as u8,
        };
        unsafe {
            *ptr &= mask;
        }
    }
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

impl Display for Castle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        write!(f, "{output}")
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
