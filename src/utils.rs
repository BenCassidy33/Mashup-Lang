use std::{collections::HashSet, string::FromUtf8Error};

pub const SPECIAL_CHARACTERS: [char; 9] = ['(', ')', '{', '}', '[', ']', ';', ',', ':'];

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
