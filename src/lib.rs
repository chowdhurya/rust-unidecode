//! The `rust-unidecode` library is a Rust port of Sean M. Burke's famous
//! [`Text::Unidecode`](http://search.cpan.org/~sburke/Text-Unidecode-1.23/lib/Text/Unidecode.pm)
//! module for Perl. It transliterates Unicode strings such as "Æneid" into pure
//! ASCII ones such as "AEneid." For a detailed explanation on the rationale
//! behind using such a library, you can refer to both the documentation of the
//! original module and
//! [this article](http://interglacial.com/~sburke/tpj/as_html/tpj22.html)
//! written by Burke in 2001.
//!
//! The data set used to translate the Unicode was ported directly from the
//! `Text::Unidecode` module using a Perl script, so `rust-unidecode` should
//! produce identical output.
//!
//! Examples
//! --------
//! ```
//! use unidecode::unidecode;
//! assert_eq!(unidecode("Æneid"), "AEneid");
//! assert_eq!(unidecode("étude"), "etude");
//! assert_eq!(unidecode("北亰"), "Bei Jing ");
//! assert_eq!(unidecode("ᔕᓇᓇ"), "shanana");
//! assert_eq!(unidecode("げんまい茶"), "genmaiCha ");
//! ```

mod data;
use data::MAPPING;

/// This function takes any Unicode string and returns an ASCII transliteration
/// of that string.
///
/// # Guarantees
///
/// * The `String` returned will be valid ASCII; the decimal representation of
///   every `char` in the string will be between 0 and 127, inclusive.
/// * Every ASCII character (0x0000 - 0x007F) is mapped to itself.
/// * All Unicode characters will translate to a string containing newlines
///   (`"\n"`) or ASCII characters in the range 0x0020 - 0x007E. So for example,
///   no Unicode character will translate to `\u{01}`. The exception is if the
///   ASCII character itself is passed in, in which case it will be mapped to
///   itself. (So `'\u{01}'` will be mapped to `"\u{01}"`.)
///
/// # Warnings
///
/// * As stated, some transliterations do produce `\n` characters.
/// * Some Unicode characters transliterate to an empty string, either on
///   purpose or because `rust-unidecode` does not know about the character.
/// * Some Unicode characters are unknown and transliterate to `"[?]"`.
/// * Many Unicode characters transliterate to multi-character strings. For
///   example, 北 is transliterated as "Bei ".
///
/// These guarantees/warnings are paraphrased from the original
/// `Text::Unidecode` documentation.
pub fn unidecode(s: &str) -> String {
    s.chars().map(|ch| unidecode_char(ch)).collect()
}

/// This function takes a single Unicode character and returns an ASCII
/// transliteration.
///
/// The warnings and guarantees of `unidecode()` apply to this function as well.
///
/// Examples
/// --------
/// ```
/// # use unidecode::unidecode_char;
/// assert_eq!(unidecode_char('Æ'), "AE");
/// assert_eq!(unidecode_char('北'), "Bei ");
/// ```
#[inline]
pub fn unidecode_char(ch: char) -> &'static str {
    MAPPING.get(ch as usize).map(|&s| s).unwrap_or("")
}

/// UniDecode Trait for idiomatic transliteration.
///
/// With this trait _unidecode_ transliteration can be done in an idiomatic fashion.
///
/// Examples
/// --------
/// ```
/// use unidecode::UniDecode;
///
/// // for string slices
/// assert_eq!("Æneid".unidecode(), "AEneid");
/// assert_eq!("вопросов".unidecode(), "voprosov");
/// assert_eq!("北亰".unidecode(), "Bei Jing ");
/// assert_eq!("ᔕᓇᓇ".unidecode(), "shanana");
/// assert_eq!("アリガトゥ".unidecode(), "arigatou");
///
/// // for chars
/// assert_eq!('™'.unidecode(), "tm");
/// assert_eq!('®'.unidecode(), "(r)");
/// assert_eq!('Æ'.unidecode(), "AE");
/// assert_eq!('é'.unidecode(), "e");
/// ```
pub trait UniDecode {
    type Output;
    /// performs _unidecode_ transliteration
    fn unidecode(&self) -> Self::Output;
}

impl UniDecode for str {
    type Output = String;
    fn unidecode(&self) -> Self::Output {
        unidecode(&self)
    }
}

impl UniDecode for char {
    type Output = &'static str;

    fn unidecode(&self) -> Self::Output {
        unidecode_char(*self)
    }
}
