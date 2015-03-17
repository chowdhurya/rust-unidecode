extern crate unidecode;
use unidecode::unidecode;

// Tests that every character outputted by the unidecode() function is valid
// ASCII.
#[test]
fn test_all_ascii() {
    use std::char;

    let valid_unicode = (0x0..0xD7FF + 1).chain(0x0E000..0x10FFFF + 1);
    for i in valid_unicode {
        match char::from_u32(i) {
            Some(ch) => {
                for ascii_ch in unidecode(&ch.to_string()).chars() {
                    let x = ascii_ch as u32;
                    if x > 127 {
                        panic!(
                            "Data contains non-ASCII character (Dec: {})",
                            x
                        );
                    }
                }
            },
            None => panic!("Test written incorrectly; invalid unicode")
        }
    }
}

// These tests were ported directly from the original `Text::Unidecode` Perl
// module.
#[test]
fn test_conversion() {
    assert_eq!(unidecode("Æneid"), "AEneid");
    assert_eq!(unidecode("étude"), "etude");
    assert_eq!(unidecode("北亰"), "Bei Jing ");
    assert_eq!(unidecode("ᔕᓇᓇ"), "shanana");
    assert_eq!(unidecode("ᏔᎵᏆ"), "taliqua");
    assert_eq!(unidecode("ܦܛܽܐܺ"), "ptu'i");
    assert_eq!(unidecode("अभिजीत"), "abhijiit");
    assert_eq!(unidecode("অভিজীত"), "abhijiit");
    assert_eq!(unidecode("അഭിജീത"), "abhijiit");
    assert_eq!(unidecode("മലയാലമ്"), "mlyaalm");
    assert_eq!(unidecode("げんまい茶"), "genmaiCha ");
}
