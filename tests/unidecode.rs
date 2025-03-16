use deunicode::*;

#[test]
/// Tests that every character outputted by the deunicode_char() function is valid ASCII.
fn test_every_char_is_ascii1() {
    for c in (0 ..= 0x1FFFF)
        .filter_map(std::char::from_u32)
        .filter_map(deunicode_char) {
        assert!(c.chars().all(|ascii_ch| {
            ascii_ch as u32 <= 127
        }));
    }
}

#[test]
/// Tests that every character outputted by the deunicode_char() function is valid ASCII.
fn test_every_char_is_ascii2() {
    for c in (0x1FFFF ..= 0x10FFFF)
        .filter_map(std::char::from_u32)
        .filter_map(deunicode_char) {
        assert!(c.chars().all(|ascii_ch| {
            ascii_ch as u32 <= 127 && ascii_ch as u8 >= b'\t'
        }));
    }
}

// These tests were ported directly from the original `Text::deunicode` Perl
// module.
#[test]
#[cfg(feature = "alloc")]
fn test_conversion() {
    assert_eq!(deunicode("✓"), "OK");
    assert_eq!(deunicode("Æneid"), "AEneid");
    assert_eq!(deunicode("étude"), "etude");
    assert_eq!(deunicode("北亰"), "Bei Jing");
    assert_eq!(deunicode("北亰city"), "Bei Jing city");
    assert_eq!(deunicode("北亰 city"), "Bei Jing city");
    assert_eq!(deunicode("北 亰 — city"), "Bei Jing -- city");
    assert_eq!(deunicode("北亰 city "), "Bei Jing city ");
    assert_eq!(deunicode("ᔕᓇᓇ"), "shanana");
    assert_eq!(deunicode("ᏔᎵᏆ"), "taliqua");
    assert_eq!(deunicode("ܦܛܽܐܺ"), "ptu'i");
    assert_eq!(deunicode("अभिजीत"), "abhijiit");
    assert_eq!(deunicode("অভিজীত"), "abhijiit");
    assert_eq!("അഭിജീത".ascii_chars().to_string(), "abhijiit");
    assert_eq!(deunicode("മലയാലമ്"), "mlyaalm");
    assert_eq!(deunicode("げんまい茶"), "genmaiCha");
    assert_eq!(deunicode("🦄☣"), "unicorn biohazard");
    assert_eq!(deunicode("🦄 ☣"), "unicorn biohazard");
    assert_eq!("🦄 ☣".ascii_chars().to_string(), "unicorn biohazard");
    assert_eq!(deunicode(" spaces "), " spaces ");
    assert_eq!(deunicode("  two  spaces  "), "  two  spaces  ");
    assert_eq!(deunicode(&[std::char::from_u32(61849).unwrap()].iter().collect::<String>()), "[?]");
    assert_eq!(deunicode_with_tofu(&[std::char::from_u32(61849).unwrap()].iter().collect::<String>(), "tofu"), "tofu");
    assert_eq!(deunicode_with_tofu_cow("\u{2713} [x]", "?"), "OK [x]");
}

#[test]
#[cfg(feature = "alloc")]
fn test_issue_7() {
    assert_eq!(deunicode("技术").to_lowercase(), "ji shu");
    assert_eq!(deunicode("评价").to_lowercase(), "ping jia");
    assert_eq!(deunicode("旅游").to_lowercase(), "lv you");
    assert_eq!("旅游".ascii_chars().to_string().to_lowercase(), "lv you");
}

#[test]
fn test_deunicode_char() {
    assert_eq!(deunicode_char('Æ'), Some("AE"));
    assert_eq!(deunicode_char('北'), Some("Bei "));
    assert_eq!(deunicode_char('亰'), Some("Jing "));
    assert_eq!(deunicode_char('ᔕ'), Some("sha"));

    assert_eq!(deunicode_char(std::char::from_u32(0x1FFFF).unwrap()), None);
    assert_eq!(deunicode_char(std::char::from_u32(0x2FFFF).unwrap()), None);
    assert_eq!(deunicode_char(std::char::from_u32(0x3FFFF).unwrap()), None);
    assert_eq!(deunicode_char(std::char::from_u32(0x4FFFF).unwrap()), None);
    assert_eq!(deunicode_char(std::char::from_u32(0x5FFFF).unwrap()), None);
    assert_eq!(deunicode_char(std::char::from_u32(0x6FFFF).unwrap()), None);
    assert_eq!(deunicode_char(std::char::from_u32(0x7FFFF).unwrap()), None);
    assert_eq!(deunicode_char(std::char::from_u32(0x8FFFF).unwrap()), None);
    assert_eq!(deunicode_char(std::char::from_u32(0x9FFFF).unwrap()), None);
    assert_eq!(deunicode_char(std::char::from_u32(0x10FFFF).unwrap()), None);
}
