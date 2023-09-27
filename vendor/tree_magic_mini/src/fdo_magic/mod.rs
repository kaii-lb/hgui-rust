// Common routines for all fdo_magic parsers

pub mod builtin;

#[derive(Debug, Clone)]
pub struct MagicRule<'a> {
    pub indent_level: u32,
    pub start_off: u32,
    pub val: &'a [u8],
    pub mask: Option<&'a [u8]>,
    pub word_len: u32,
    pub region_len: u32,
}

pub mod check;
pub mod ruleset;
