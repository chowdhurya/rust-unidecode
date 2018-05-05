extern crate deunicode;
use deunicode::{deunicode, deunicode_char};

// Tests that every character outputted by the deunicode() function is valid
// ASCII.
#[test]
fn test_all_ascii() {
    use std::char;

    let valid_unicode = (0x0..0xD7FF + 1).chain(0x0E000..0x10FFFF + 1);
    for i in valid_unicode {
        match char::from_u32(i) {
            Some(ch) => {
                for ascii_ch in deunicode(&ch.to_string()).chars() {
                    let x = ascii_ch as u32;
                    if x > 127 {
                        panic!(
                            "Data contains non-ASCII character (Dec: {})",
                            x
                        );
                    }
                }
            },
            None => panic!("Test written incorrectly; invalid Unicode")
        }
    }
}

// These tests were ported directly from the original `Text::deunicode` Perl
// module.
#[test]
fn test_conversion() {
    assert_eq!(deunicode("Æneid"), "AEneid");
    assert_eq!(deunicode("étude"), "etude");
    assert_eq!(deunicode("北亰"), "Bei Jing");
    assert_eq!(deunicode("北亰city"), "Bei Jing city");
    assert_eq!(deunicode("北亰 city"), "Bei Jing city");
    assert_eq!(deunicode("北 亰 city"), "Bei Jing city");
    assert_eq!(deunicode("北亰 city "), "Bei Jing city ");
    assert_eq!(deunicode("ᔕᓇᓇ"), "shanana");
    assert_eq!(deunicode("ᏔᎵᏆ"), "taliaqu");
    assert_eq!(deunicode("ܦܛܽܐܺ"), "ptu'i");
    assert_eq!(deunicode("अभिजीत"), "abhijiit");
    assert_eq!(deunicode("অভিজীত"), "abhijiit");
    assert_eq!(deunicode("അഭിജീത"), "abhijiit");
    assert_eq!(deunicode("മലയാലമ്"), "mlyaalm");
    assert_eq!(deunicode("げんまい茶"), "genmaiCha");
    assert_eq!(deunicode("🦄☣"), "unicorn face biohazard");
    assert_eq!(deunicode("🦄 ☣"), "unicorn face biohazard");
    assert_eq!(deunicode(" spaces "), " spaces ");
    assert_eq!(deunicode("  two  spaces  "), "  two  spaces  ");
}

#[test]
fn test_deunicode_char() {
    assert_eq!(deunicode_char('Æ'), "AE");
    assert_eq!(deunicode_char('北'), "Bei ");
    assert_eq!(deunicode_char('亰'), "Jing ");
    assert_eq!(deunicode_char('ᔕ'), "sha");
}
