extern crate deunicode;
use deunicode::*;

#[test]
/// Tests that every character outputted by the deunicode_char() function is valid ASCII.
fn test_every_char_is_ascii() {
    use std::char;

    for i in 0 ..= 0x10FFFF {
        match char::from_u32(i) {
            Some(ch) => {
                if let Some(c) = from_char(ch) {
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
fn test_lossy_conversion() {
    assert_eq!(from_str_lossy("Æneid"), "AEneid");
    assert_eq!(from_str_lossy("étude"), "etude");
    assert_eq!(from_str_lossy("北亰"), "Bei Jing");
    assert_eq!(from_str_lossy("北亰city"), "Bei Jing city");
    assert_eq!(from_str_lossy("北亰 city"), "Bei Jing city");
    assert_eq!(from_str_lossy("北 亰 city"), "Bei Jing city");
    assert_eq!(from_str_lossy("北亰 city "), "Bei Jing city ");
    assert_eq!(from_str_lossy("ᔕᓇᓇ"), "shanana");
    assert_eq!(from_str_lossy("ᏔᎵᏆ"), "taliaqu");
    assert_eq!(from_str_lossy("ܦܛܽܐܺ"), "ptu'i");
    assert_eq!(from_str_lossy("अभिजीत"), "abhijiit");
    assert_eq!(from_str_lossy("অভিজীত"), "abhijiit");
    assert_eq!(from_str_lossy("അഭിജീത"), "abhijiit");
    assert_eq!(from_str_lossy("മലയാലമ്"), "mlyaalm");
    assert_eq!(from_str_lossy("げんまい茶"), "genmaiCha");
    assert_eq!(from_str_lossy("🦄☣"), "unicorn face biohazard");
    assert_eq!(from_str_lossy("🦄 ☣"), "unicorn face biohazard");
    assert_eq!(from_str_lossy(" spaces "), " spaces ");
    assert_eq!(from_str_lossy("  two  spaces  "), "  two  spaces  ");
    assert_eq!(from_str_lossy(&[std::char::from_u32(849).unwrap()].iter().collect::<String>()), "[?]");
    assert_eq!(from_str_with_tofu(&[std::char::from_u32(849).unwrap()].iter().collect::<String>(), "tofu"), "tofu");
}

#[test]
fn test_conversion() {
    assert_eq!(from_str("Æneid"), Some("AEneid".into()));
    assert_eq!(from_str("étude"), Some("etude".into()));
    assert_eq!(from_str("北亰"), Some("Bei Jing".into()));
    assert_eq!(from_str("北亰city"), Some("Bei Jing city".into()));
    assert_eq!(from_str("北亰 city"), Some("Bei Jing city".into()));
    assert_eq!(from_str("北 亰 city"), Some("Bei Jing city".into()));
    assert_eq!(from_str("北亰 city "), Some("Bei Jing city ".into()));
    assert_eq!(from_str("ᔕᓇᓇ"), Some("shanana".into()));
    assert_eq!(from_str("ᏔᎵᏆ"), Some("taliaqu".into()));
    assert_eq!(from_str("ܦܛܽܐܺ"), Some("ptu'i".into()));
    assert_eq!(from_str("अभिजीत"), Some("abhijiit".into()));
    assert_eq!(from_str("অভিজীত"), Some("abhijiit".into()));
    assert_eq!(from_str("അഭിജീത"), Some("abhijiit".into()));
    assert_eq!(from_str("മലയാലമ്"), Some("mlyaalm".into()));
    assert_eq!(from_str("げんまい茶"), Some("genmaiCha".into()));
    assert_eq!(from_str("🦄☣"), Some("unicorn face biohazard".into()));
    assert_eq!(from_str("🦄 ☣"), Some("unicorn face biohazard".into()));
    assert_eq!(from_str(" spaces "), Some(" spaces ".into()));
    assert_eq!(from_str("  two  spaces  "), Some("  two  spaces  ".into()));
    assert_eq!(from_str(&[std::char::from_u32(849).unwrap()].iter().collect::<String>()), None);
}

#[test]
fn test_deunicode_char() {
    assert_eq!(from_char('Æ'), Some("AE"));
    assert_eq!(from_char('北'), Some("Bei "));
    assert_eq!(from_char('亰'), Some("Jing "));
    assert_eq!(from_char('ᔕ'), Some("sha"));
    assert_eq!(from_char(std::char::from_u32(849).unwrap()), None);
}
