rust-unidecode
==============

The `rust-unidecode` library is a Rust port of Sean M. Burke's famous
[`Text::Unidecode`](http://search.cpan.org/~sburke/Text-Unidecode-1.23/lib/Text/Unidecode.pm)
module for Perl. It transliterates Unicode strings such as "Æneid" into pure
ASCII ones such as "AEneid." For a detailed explanation on the rationale behind
using such a library, you can refer to both the documentation of the original
module and
[this article](http://interglacial.com/~sburke/tpj/as_html/tpj22.html) written
by Burke in 2001.

The data set used to translate the Unicode was ported directly from the
`Text::Unidecode` module using a Perl script, so `rust-unidecode` should produce
identical output.

Examples
--------
```rust
extern crate unidecode;
use unidecode::unidecode;

assert_eq!(unidecode("Æneid"), "AEneid");
assert_eq!(unidecode("étude"), "etude");
assert_eq!(unidecode("北亰"), "Bei Jing");
assert_eq!(unidecode("ᔕᓇᓇ"), "shanana");
assert_eq!(unidecode("げんまい茶"), "genmaiCha ");
```

Guarantees and Warnings
-----------------------
Here are some guarantees you have when calling `unidecode()`:
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
    or because `rust-unidecode` does not know about the character.
  * Some Unicode characters are unknown and transliterate to `"[?]"`.
  * Many Unicode characters transliterate to multi-character strings. For
    example, 北 is transliterated as "Bei ".

This information was paraphrased from the original `Text::Unidecode`
documentation.
