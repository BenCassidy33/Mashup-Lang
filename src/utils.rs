pub const SPECIAL_CHARACTERS: [char; 8] = ['(', ')', '{', '}', '[', ']', ';', ','];

pub trait SpecialImplmentation {
    fn is_special_character(ch: u8) -> bool;
}

impl SpecialImplmentation for u8 {
    fn is_special_character(ch: u8) -> bool {
        return SPECIAL_CHARACTERS.contains(&(ch as char));
    }
}
