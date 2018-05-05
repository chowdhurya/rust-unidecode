# deunicode

[Documentation](https://docs.rs/deunicode/)

The `deunicode` library transliterates Unicode strings such as "Æneid" into pure
ASCII ones such as "AEneid."

It started as a Rust port of [`Text::Unidecode`](http://search.cpan.org/~sburke/Text-Unidecode-1.30/lib/Text/Unidecode.pm) Perl module, and was extended to support emoji.

Examples
--------
```rust
extern crate deunicode;
use deunicode::deunicode;

assert_eq!(deunicode("Æneid"), "AEneid");
assert_eq!(deunicode("étude"), "etude");
assert_eq!(deunicode("北亰"), "Bei Jing");
assert_eq!(deunicode("ᔕᓇᓇ"), "shanana");
assert_eq!(deunicode("げんまい茶"), "genmaiCha");
assert_eq!(deunicode("🦄☣"), "unicorn face biohazard");
```

Guarantees and Warnings
-----------------------
Here are some guarantees you have when calling `deunicode()`:
  * The `String` returned will be valid ASCII; the decimal representation of
    every `char` in the string will be between 0 and 127, inclusive.
  * Every ASCII character (0x0000 - 0x007F) is mapped to itself.
  * All Unicode characters will translate to a string containing newlines
    (`"\n"`) or ASCII characters in the range 0x0020 - 0x007E. So for example,
    no Unicode character will translate to `\u{01}`. The exception is if the
    ASCII character itself is passed in, in which case it will be mapped to
    itself. (So `'\u{01}'` will be mapped to `"\u{01}"`.)

There are, however, some things you should keep in mind:
  * As stated, some transliterations do produce `\n` characters.
  * Some Unicode characters transliterate to an empty string, either on purpose
    or because `deunicode` does not know about the character.
  * Some Unicode characters are unknown and transliterate to `"[?]"`.
  * Many Unicode characters transliterate to multi-character strings. For
    example, 北 is transliterated as "Bei ".
  * Han characters are mapped to Mandarin, and will be mostly illegible to Japanese readers.

Unicode data
------------
 * [`Text::Unidecode`](http://search.cpan.org/~sburke/Text-Unidecode-1.30/lib/Text/Unidecode.pm) by Sean M. Burke
 * [Unicodey](https://unicodey.com) by Cal Henderson

For a detailed explanation on the rationale behind the original
dataset, refer to [this article](http://interglacial.com/~sburke/tpj/as_html/tpj22.html) written
by Burke in 2001.
