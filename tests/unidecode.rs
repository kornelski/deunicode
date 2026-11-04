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

fn assert_ignored_mapping_spacing(input: &str, expected: &str, parts: &[Option<&str>]) {
    let mut iter = input.ascii_chars();
    let actual: Vec<_> = iter.clone().collect();
    assert_eq!(actual.as_slice(), parts);
    assert_eq!(format!("{}", iter.clone()), expected);
    assert_eq!(iter.clone().flatten().collect::<String>(), expected);

    for (index, part) in parts.iter().enumerate() {
        let remaining = parts.len() - index;
        let (lower, upper) = iter.size_hint();
        assert!(lower <= remaining);
        assert!(upper.map_or(true, |upper| upper >= remaining));
        assert_eq!(iter.clone().count(), remaining);
        assert_eq!(iter.next(), Some(*part));
    }
    assert_eq!(iter.clone().count(), 0);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    #[cfg(feature = "alloc")]
    {
        assert_eq!(deunicode(input), expected);
        assert_eq!(deunicode_with_tofu(input, "?"), expected);
        assert_eq!(deunicode_with_tofu_cow(input, "?"), expected);
    }
}

#[test]
fn ignored_mapping_spacing_combining_suffix() {
    assert_ignored_mapping_spacing("北\u{301}", "Bei", &[Some("Bei"), Some("")]);
}

#[test]
fn ignored_mapping_spacing_variation_suffix() {
    assert_ignored_mapping_spacing("☃\u{fe0f}", "snowman", &[Some("snowman"), Some("")]);
}

#[test]
fn ignored_mapping_spacing_two_empty_suffixes() {
    assert_ignored_mapping_spacing(
        "北\u{301}\u{fe0f}",
        "Bei",
        &[Some("Bei"), Some(""), Some("")],
    );
}

#[test]
fn ignored_mapping_spacing_original_trailing_space() {
    assert_ignored_mapping_spacing("北\u{301} ", "Bei ", &[Some("Bei"), Some(""), Some(" ")]);
}

#[test]
fn ignored_mapping_spacing_original_repeated_spaces() {
    assert_ignored_mapping_spacing(
        "北\u{301}  ",
        "Bei  ",
        &[Some("Bei"), Some(""), Some(" "), Some(" ")],
    );
}

#[test]
fn ignored_mapping_spacing_space_before_word() {
    assert_ignored_mapping_spacing(
        "北\u{301} 亰",
        "Bei Jing",
        &[Some("Bei"), Some(""), Some(" "), Some("Jing")],
    );
}

#[test]
fn ignored_mapping_spacing_two_empty_before_word() {
    assert_ignored_mapping_spacing(
        "北\u{301}\u{fe0f} 亰",
        "Bei Jing",
        &[Some("Bei"), Some(""), Some(""), Some(" "), Some("Jing")],
    );
}

#[test]
fn ignored_mapping_spacing_controls() {
    assert_ignored_mapping_spacing("北", "Bei", &[Some("Bei")]);
    assert_ignored_mapping_spacing("北\u{301}x", "Bei x", &[Some("Bei "), Some(""), Some("x")]);
    assert_ignored_mapping_spacing(
        "北\u{301}亰",
        "Bei Jing",
        &[Some("Bei "), Some(""), Some("Jing")],
    );
    assert_eq!(
        "  two  spaces  ".ascii_chars().flatten().collect::<String>(),
        "  two  spaces  "
    );
    assert_eq!(deunicode_char('北'), Some("Bei "));
    assert_eq!(deunicode_char('☃'), Some("snowman "));
    assert_eq!(deunicode_char('\u{301}'), Some(""));
    assert_eq!(deunicode_char('\u{fe0f}'), Some(""));

    let unknown = "北\u{301}\u{10ffff}";
    let parts: Vec<_> = unknown.ascii_chars().collect();
    assert_eq!(parts, vec![Some("Bei "), Some(""), None]);
    assert_eq!(format!("{}", unknown.ascii_chars()), "Bei \u{fffd}");
    assert_eq!(unknown.ascii_chars().count(), 3);
    #[cfg(feature = "alloc")]
    {
        assert_eq!(deunicode(unknown), "Bei [?]");
        assert_eq!(deunicode_with_tofu(unknown, "?"), "Bei ?");
        assert_eq!(deunicode_with_tofu(unknown, ""), "Bei ");
        assert_eq!(deunicode_with_tofu(unknown, " "), "Bei  ");
    }
}
