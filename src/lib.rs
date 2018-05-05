//! Please see README.md for more info

const MAPPING: &str = include_str!("mapping.txt");
const POINTERS: &[u8] = include_bytes!("pointers.bin");

/// This function takes any Unicode string and returns an ASCII transliteration
/// of that string.
///
/// Guarantees and Warnings
/// -----------------------
/// Here are some guarantees you have when calling `deunicode()`:
///   * The `String` returned will be valid ASCII; the decimal representation of
///     every `char` in the string will be between 0 and 127, inclusive.
///   * Every ASCII character (0x0000 - 0x007F) is mapped to itself.
///   * All Unicode characters will translate to a string containing newlines
///     (`"\n"`) or ASCII characters in the range 0x0020 - 0x007E. So for example,
///     no Unicode character will translate to `\u{01}`. The exception is if the
///     ASCII character itself is passed in, in which case it will be mapped to
///     itself. (So `'\u{01}'` will be mapped to `"\u{01}"`.)
///
/// There are, however, some things you should keep in mind:
///   * As stated, some transliterations do produce `\n` characters.
///   * Some Unicode characters transliterate to an empty string, either on purpose
///     or because `deunicode` does not know about the character.
///   * Some Unicode characters are unknown and transliterate to `"[?]"`.
///   * Many Unicode characters transliterate to multi-character strings. For
///     example, 北 is transliterated as "Bei ".
///   * Han characters are mapped to Mandarin, and will be mostly illegible to Japanese readers.
pub fn deunicode(s: &str) -> String {
    s.chars().map(|ch| deunicode_char(ch)).collect()
}

/// This function takes a single Unicode character and returns an ASCII
/// transliteration.
///
/// The warnings and guarantees of `deunicode()` apply to this function as well.
///
/// Examples
/// --------
/// ```ignore
/// assert_eq!(deunicode_char('Æ'), "AE");
/// assert_eq!(deunicode_char('北'), "Bei");
/// ```
#[inline]
pub fn deunicode_char(ch: char) -> &'static str {
    let ptr_pos = ch as usize * 3;

    // pointers format is {
    //    union {
    //       char: [u8;1]
    //       char: [u8;2]
    //       index: u16
    //    }
    //    len: u8
    // }
    if let Some(&len) = POINTERS.get(ptr_pos+2) {
        if len <= 2 {
            // if length is 1 or 2, then the "pointer" data is used to store the char
            std::str::from_utf8(&POINTERS[ptr_pos..ptr_pos + len as usize]).unwrap()
        } else {
            let map_pos = (POINTERS[ptr_pos] as u16 | (POINTERS[ptr_pos+1] as u16) << 8) as usize;
            &MAPPING[map_pos..map_pos + len as usize]
        }
    } else {
        ""
    }
}
