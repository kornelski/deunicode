//! The `deunicode` library transliterates Unicode strings such as "Æneid" into pure
//! ASCII ones such as "AEneid."
//!
//! It started as a Rust port of [`Text::Unidecode`](http://search.cpan.org/~sburke/Text-Unidecode-1.30/lib/Text/Unidecode.pm) Perl module, and was extended to support emoji.
//!
//! See [README](https://github.com/kornelski/deunicode/blob/master/README.md) for more info.
//!
//! Examples
//! --------
//! ```rust
//! extern crate deunicode;
//! use deunicode::from_str_lossy;
//!
//! assert_eq!(from_str_lossy("Æneid"), "AEneid");
//! assert_eq!(from_str_lossy("étude"), "etude");
//! assert_eq!(from_str_lossy("北亰"), "Bei Jing");
//! assert_eq!(from_str_lossy("ᔕᓇᓇ"), "shanana");
//! assert_eq!(from_str_lossy("げんまい茶"), "genmaiCha");
//! assert_eq!(from_str_lossy("🦄☣"), "unicorn face biohazard");
//! ```

const MAPPING: &str = include_str!("mapping.txt");

#[repr(C)]
#[derive(Copy, Clone)]
struct Ptr {
    /// if len <= 2, it's the string itself,
    /// otherwise it's an u16 offset into MAPPING
    chr: [u8; 2],
    len: u8,
}

/// POINTERS format is described by struct Ptr
const POINTERS: &[u8] = include_bytes!("pointers.bin");

/// This function takes any Unicode string and returns an ASCII transliteration
/// of that string.
///
/// Guarantees and Warnings
/// -----------------------
/// Here are some guarantees you have when calling `from_str_lossy()`:
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
///     or because `from_str_lossy` does not know about the character.
///   * Some Unicode characters are unknown and transliterate to `"[?]"`.
///   * Many Unicode characters transliterate to multi-character strings. For
///     example, 北 is transliterated as "Bei ".
///   * Han characters are mapped to Mandarin, and will be mostly illegible to Japanese readers.
#[inline]
pub fn from_str_lossy(s: &str) -> String {
    from_str_with_tofu(s, "[?]")
}

/// Same as `from_str_lossy`, but returns `None` if unknown characters are detected.
pub fn from_str(s: &str) -> Option<String> {
    let mut out = String::with_capacity(s.len());
    let mut had_space = false;
    for ch in s.chars().map(|ch| from_char(ch)) {
        let ch = ch?;
        // don't add space after transliteration with a space
        if !had_space || " " != ch {
            out.push_str(ch);
        }
        had_space = ch.len() > 1 && ch.as_bytes()[ch.len()-1] == b' ';
    }
    if had_space { // transliteration uses spaces to avoid words joining together
        out.pop();
    }
    Some(out)
}

/// Same as `from_str_lossy`, but unknown characters can be replaced with a custom string.
///
/// "Tofu" is a nickname for a replacement character, which in Unicode fonts usually
/// looks like a block of tofu.
pub fn from_str_with_tofu(s: &str, custom_placeholder: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut had_space = false;
    for ch in s.chars().map(|ch| from_char(ch).unwrap_or(custom_placeholder)) {
        // don't add space after transliteration with a space
        if !had_space || " " != ch {
            out.push_str(ch);
        }
        had_space = ch.len() > 1 && ch.as_bytes()[ch.len()-1] == b' ';
    }
    if had_space { // transliteration uses spaces to avoid words joining together
        out.pop();
    }
    out
}

/// This function takes a single Unicode character and returns an ASCII
/// transliteration.
///
/// The warnings and guarantees of `from_str_lossy()` apply to this function as well.
///
/// Examples
/// --------
/// ```rust
/// # extern crate deunicode;
/// # use deunicode::from_char;
/// assert_eq!(from_char('Æ'), Some("AE"));
/// assert_eq!(from_char('北'), Some("Bei "));
/// ```
#[inline]
pub fn from_char(ch: char) -> Option<&'static str> {
    // when using the global directly, LLVM fails to remove bounds checks
    let pointers: &'static [Ptr] = unsafe {
        std::slice::from_raw_parts(POINTERS.as_ptr() as *const Ptr, POINTERS.len()/3)
    };

    if let Some(p) = pointers.get(ch as usize) {
        // if length is 1 or 2, then the "pointer" data is used to store the char
        if p.len <= 2 {
            // safe, because we're returning only ASCII
            unsafe {
                Some(std::str::from_utf8_unchecked(&p.chr[..p.len as usize]))
            }
        } else {
            let map_pos = (p.chr[0] as u16 | (p.chr[1] as u16) << 8) as usize;
            // unknown characters are intentionally mapped to out of range length
            MAPPING.get(map_pos..map_pos + p.len as usize)
        }
    } else {
        None
    }
}
