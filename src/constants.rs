pub const ESCAPE_CHARACTER: char = '\\';


pub const INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_LOWER: u8 = 191;
pub const INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_UPPER: u8 = 211;
pub const INTEGER_SMALL_TOKEN_OFFSET: i16 = 201;

pub const INTEGER_SMALL_TOKENS : [char; 19] = ['À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï', 'Ð', 'Ñ', 'Ò'];
pub const INTEGER_SMALL_TOKEN_ELEMENT_OFFSET: i64 = 9;
pub const INTEGER_SMALL_EXCLUSIVE_BOUND_LOWER: i64 = -10;
pub const INTEGER_SMALL_EXCLUSIVE_BOUND_UPPER: i64 = 10;
pub const BASE_62: [char; 62] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

pub const DELIMITING_TOKENS_THRESHOLD: u8 = 122;

pub const INTEGER_TOKEN: char = '¢';
pub const UNREFERENCED_INTEGER_TOKEN: char = '¤';
pub const REF_INTEGER_TOKEN: char = 'º';

pub const STRING_TOKEN: char = '¨';
pub const UNREFERENCED_STRING_TOKEN: char = '´';
pub const REF_STRING_TOKEN: char = 'ß';

pub const DATE_TOKEN: char = 'ø';
pub const REF_DATE_TOKEN: char = '×';
pub const UNREFERENCED_DATE_TOKEN: char = '¿';
pub const LP_DATE_TOKEN: char = '±';
pub const REF_LP_DATE_TOKEN: char = 'ü';
pub const UNREFERENCED_LP_DATE_TOKEN: char = 'ÿ';
pub const DATE_LOW_PRECISION: i64 = 100;

pub const FLOAT_TOKEN: char = '£';
pub const UNREFERENCED_FLOAT_TOKEN: char = '¥';
pub const REF_FLOAT_TOKEN: char = 'Ý';
pub const FLOAT_FULL_PRECISION_DELIMITER: char = ',';
pub const FLOAT_REDUCED_PRECISION_DELIMITER: char = '.';
pub const FLOAT_COMPRESSION_PRECISION: f64 = 1000_f64;

pub const ARRAY_START_TOKEN: char = '|';
pub const ARRAY_END_TOKEN: char = '÷';
pub const OBJECT_START_TOKEN: char = '{';
pub const OBJECT_END_TOKEN: char = '}';

pub const NULL_TOKEN: char = '§';
pub const BOOLEAN_TRUE_TOKEN: char = '»';
pub const BOOLEAN_FALSE_TOKEN: char = '«';

pub const REFERENCE_HEADER_LENGTH: u8 = 1;
