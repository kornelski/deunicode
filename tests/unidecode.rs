use deunicode::*;

#[test]
/// Tests that every character outputted by the deunicode_char() function is valid ASCII.
fn test_every_char_is_ascii() {
    use std::char;

    for i in 0 ..= 0x10FFFF {
        match char::from_u32(i) {
            Some(ch) => {
                if let Some(c) = deunicode_char(ch) {
                    for ascii_ch in c.chars() {
                        let x = ascii_ch as u32;
                        if x > 127 {
                            panic!(
                                "Data contains non-ASCII character (Dec: {})",
                                x
                            );
                        }
                    }
                }
            },
            _ => {}
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
    assert_eq!(deunicode("北 亰 — city"), "Bei Jing -- city");
    assert_eq!(deunicode("北亰 city "), "Bei Jing city ");
    assert_eq!(deunicode("ᔕᓇᓇ"), "shanana");
    assert_eq!(deunicode("ᏔᎵᏆ"), "taliaqu");
    assert_eq!(deunicode("ܦܛܽܐܺ"), "ptu'i");
    assert_eq!(deunicode("अभिजीत"), "abhijiit");
    assert_eq!(deunicode("অভিজীত"), "abhijiit");
    assert_eq!(deunicode("അഭിജീത"), "abhijiit");
    assert_eq!(deunicode("മലയാലമ്"), "mlyaalm");
    assert_eq!(deunicode("げんまい茶"), "genmaiCha");
    assert_eq!(deunicode("🦄☣"), "unicorn biohazard");
    assert_eq!(deunicode("🦄 ☣"), "unicorn biohazard");
    assert_eq!(deunicode(" spaces "), " spaces ");
    assert_eq!(deunicode("  two  spaces  "), "  two  spaces  ");
    assert_eq!(deunicode(&[std::char::from_u32(849).unwrap()].iter().collect::<String>()), "[?]");
    assert_eq!(deunicode_with_tofu(&[std::char::from_u32(849).unwrap()].iter().collect::<String>(), "tofu"), "tofu");
}

#[test]
fn test_deunicode_char() {
    assert_eq!(deunicode_char('Æ'), Some("AE"));
    assert_eq!(deunicode_char('北'), Some("Bei "));
    assert_eq!(deunicode_char('亰'), Some("Jing "));
    assert_eq!(deunicode_char('ᔕ'), Some("sha"));
    assert_eq!(deunicode_char(std::char::from_u32(849).unwrap()), None);
}
