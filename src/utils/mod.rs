pub mod macros;
pub mod methods;
pub mod tokens;

pub use tokens::*;

pub const SPECIAL_CHARACTERS: [char; 10] = ['(', ')', '{', '}', '[', ']', ';', ',', ':', '.'];

pub trait SpecialImplmentation {
    fn is_special_character(ch: u8) -> bool;
}

impl SpecialImplmentation for u8 {
    fn is_special_character(ch: u8) -> bool {
        return SPECIAL_CHARACTERS.contains(&(ch as char));
    }
}

pub trait VecU8Impl {
    fn to_string(&self) -> Result<String, std::string::FromUtf8Error>;
}

impl VecU8Impl for Vec<u8> {
    fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        return String::from_utf8(self.clone());
    }
}

pub trait Colors {
    fn red(&self) -> String;
    fn yellow(&self) -> String;
    fn green(&self) -> String;
}

impl Colors for &'static str {
    fn red(&self) -> String {
        return format!("\x1b[91m{}\x1b[0m", self);
    }

    fn yellow(&self) -> String {
        return format!("\x1b[93m{}\x1b[0m", self);
    }

    fn green(&self) -> String {
        return format!("\x1b[92m{}\x1b[0m", self);
    }
}
