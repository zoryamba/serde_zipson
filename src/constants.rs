pub const ESCAPE_CHARACTER: char = '\\';


pub const INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_LOWER: u8 = 191;
pub const INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_UPPER: u8 = 211;
pub const INTEGER_SMALL_TOKEN_OFFSET: i16 = 201;

pub const INTEGER_SMALL_TOKENS : [char; 19] = ['À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï', 'Ð', 'Ñ', 'Ò'];

pub const DELIMITING_TOKENS_THRESHOLD: u8 = 122;

pub const INTEGER_TOKEN: char = '¢';
pub const UNREFERENCED_INTEGER_TOKEN: char = '¤';
pub const REF_INTEGER_TOKEN: char = 'º';

pub const STRING_TOKEN: char = '¨';
pub const UNREFERENCED_STRING_TOKEN: char = '´';
pub const REF_STRING_TOKEN: char = 'ß';

pub const ARRAY_START_TOKEN: char = '|';
pub const ARRAY_END_TOKEN: char = '÷';
pub const NULL_TOKEN: char = '§';
pub const BOOLEAN_TRUE_TOKEN: char = '»';
pub const BOOLEAN_FALSE_TOKEN: char = '«';