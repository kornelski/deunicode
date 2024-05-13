//! Takes data.rs and makes pointers.bin & mapping.txt data files


#[macro_use] extern crate serde_derive;

mod data;
use unic_ucd_category::GeneralCategory;
use crate::data::MAPPING;
use std::collections::HashMap;

const UNKNOWN_CHAR: &'static str = "\0\0\0";

use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Deserialize)]
struct Emoji1 {
    emoji: String,
    name: String,
    #[serde(default)]
    shortname: String,
}

#[derive(Deserialize)]
struct Gemoji {
    emoji: Option<String>,
    aliases: Vec<String>,
}

#[derive(Deserialize)]
struct Emoji2 {
    unified: String,
    short_name: String,
}

fn emojiname(s: &str) -> String {
    if s.starts_with("skin-") { // skip skin tones
        return String::new();
    }
    let mut s = s.replace('_'," ");
    s.push(' ');
    s
}

fn main() {
    // get shortest names out of emoji data
    let emoji2 = serde_json::from_slice::<Vec<Emoji2>>(&fs::read("emoji.json").expect("emoji.json")).unwrap().iter()
        .filter_map(|e| usize::from_str_radix(&e.unified, 16).ok().map(|n| (n,emojiname(&e.short_name))))
        .collect::<Vec<_>>();

    // get shortest names out of emoji data
    let emoji1 = serde_json::from_slice::<Vec<Emoji1>>(&fs::read("emoji1.json").expect("emoji1.json")).unwrap().iter()
        .filter(|e| e.emoji.chars().count() == 1)
        .filter(|e| e.name.len() > 0 || e.shortname.len() > 0)
        .map(|e| {
            let ch = e.emoji.chars().next().unwrap() as usize;
            let shortname = e.shortname.trim_matches(':');
            if shortname.len() > 0 && shortname.len() < e.name.len() {
                (ch, emojiname(shortname))
            } else {
                (ch, emojiname(&e.name))
            }
        })
        .collect::<Vec<_>>();

    let gemoji = serde_json::from_slice::<Vec<Gemoji>>(&fs::read("gemoji/db/emoji.json").expect("gemoji")).unwrap();
    let gemoji = gemoji.iter()
        .filter_map(|e| {
            if let Some(ref emoji) = e.emoji {
                if emoji.chars().count() == 1 {
                    let ch = emoji.chars().next().unwrap() as usize;
                    return Some((ch, &e.aliases))
                }
            }
            None
        })
        .flat_map(|(ch, aliases)| {
            aliases.into_iter().map(move |name| (ch, emojiname(name)))
        })
        .collect::<Vec<_>>();

    // merge shortest names
    let mut all_codepoints: Vec<_> = MAPPING.iter().copied().map(|ch| {
        // old data marks unknown as "[?]"
        if ch != "[?] " && ch != "[?]" {ch} else {UNKNOWN_CHAR}
    }).collect();

    if all_codepoints.len() < 140000 { all_codepoints.resize(140000, UNKNOWN_CHAR); }

    all_codepoints['术' as usize] = "Shu ";
    all_codepoints['价' as usize] = "Jia ";
    all_codepoints['旅' as usize] = "Lv ";
    all_codepoints['什' as usize] = "Shen ";
    all_codepoints['么' as usize] = "Me ";
    all_codepoints['❗' as usize] = "!";
    all_codepoints['❕' as usize] = "!";
    all_codepoints['❓' as usize] = "?";
    all_codepoints['❔' as usize] = "?";
    all_codepoints['➕' as usize] = "+";
    all_codepoints['➖' as usize] = "-";
    all_codepoints['➗' as usize] = "/";
    all_codepoints['🟰' as usize] = "=";
    all_codepoints['✖' as usize] = "x";
    all_codepoints['💲' as usize] = "$";
    all_codepoints['💵' as usize] = "$";
    all_codepoints['🌟' as usize] = "*";
    all_codepoints['★' as usize] = "*";
    all_codepoints['☀' as usize] = "*";
    all_codepoints['☹' as usize] = ":(";
    all_codepoints['☺' as usize] = ":)";
    all_codepoints['✳' as usize] = "*";
    all_codepoints['❇' as usize] = "*";
    all_codepoints['✴' as usize] = "*";
    all_codepoints['⭐' as usize] = "*";
    all_codepoints['☻' as usize] = ":)";

    for &(ch, ref name) in gemoji.iter().chain(emoji1.iter()).chain(emoji2.iter()) {
        if all_codepoints.len() <= ch {
            all_codepoints.resize(ch as usize+1, UNKNOWN_CHAR);
        }
        if "" == all_codepoints[ch] || "[?]" == all_codepoints[ch] || UNKNOWN_CHAR == all_codepoints[ch] || name.len() < all_codepoints[ch].len() {
            all_codepoints[ch] = name;
        }
    }

    for (mut name, ch) in emojis::iter().filter(|e| e.as_str().chars().count() == 1)
        .filter_map(|e| Some((e.shortcode().unwrap_or(e.name()), e.as_str().chars().next()? as usize))) {
        if all_codepoints.len() <= ch {
            all_codepoints.resize(ch as usize+1, UNKNOWN_CHAR);
        }
        if "" == all_codepoints[ch] || "[?]" == all_codepoints[ch] || UNKNOWN_CHAR == all_codepoints[ch] {
            let new_name = format!("{} ", name.trim().replace('_', " "));
            name = Box::leak(new_name.into_boxed_str());
            all_codepoints[ch] = name;
        }
    }

    for i in 255..all_codepoints.len() {
        let Some(codepoint) = std::char::from_u32(i as u32) else { continue; };
        let ch = all_codepoints[i];
        if ch == UNKNOWN_CHAR {
            let mut any = any_ascii::any_ascii_char(codepoint).trim_matches(':');
            if GeneralCategory::of(codepoint) == GeneralCategory::OtherLetter {
                if any.as_bytes().iter().any(|b| b.is_ascii_digit()) {
                    // hieroglyphs are just "A123"
                    any = "";
                }
            }
            if any != "" {
                // we use spaces instead of underscores in emoji
                all_codepoints[i] = if any.chars().any(|c| c.is_alphabetic()) && any.chars().any(|c| c == '_') {
                    let ch: String = any.chars().map(|c| if c == '_' {' '} else {c}).collect();
                    Box::leak(ch.into_boxed_str())
                } else {
                    any
                };
            } else {
                let mut s = String::new();
                let mut changed = false;
                unicode_normalization::char::decompose_compatible(codepoint, |denorm| {
                    if denorm as usize != i { changed = true; }
                    all_codepoints.get(denorm as usize).map(|c| s.push_str(c));
                });
                if changed && !s.trim().is_empty() && s.bytes().all(|c| c < 255) {
                    all_codepoints[i] = Box::leak(s.into_boxed_str());
                }
            }
        } else  if ch.starts_with("[d") {
            // clean up [d123]
            all_codepoints[i] = ch.trim_start_matches('[').trim_end_matches(']');
        };
    }

    let sequences = std::fs::read_to_string("emoji-sequences.txt").unwrap();
    for line in sequences.lines().map(|l| l.trim()).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let (name, rest) = line.split(';').nth(2).unwrap().split_once('#').unwrap();
        for (n, e) in name.split("..").zip(rest.split("..")) {
            let e = e.trim_matches(|c: char| (c as u32) < 128);
            let e = e.chars().filter(|&c| {
                !matches!(c as u32, 127995..=127999 | 65039)
            }).collect::<Vec<_>>();
            if e.len() != 1 {
                continue;
            }
            let ch = e[0] as usize;
            if ch == 8419 || ch == 917536 {
                continue;
            }
            if all_codepoints.len() <= ch {
                all_codepoints.resize(ch as usize+1, UNKNOWN_CHAR);
            }
            if "" == all_codepoints[ch] || "[?]" == all_codepoints[ch] || UNKNOWN_CHAR == all_codepoints[ch] {
                let new_name = emojiname(&n.trim().replace('_', " ")
                    .trim_end_matches(" face")
                    .trim_end_matches(" hand")
                    .trim_end_matches(" sign")
                    .trim_start_matches("circled ")
                    .to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>());
                all_codepoints[ch] = Box::leak(new_name.into_boxed_str());
            }
        }
    }

    let sequences = std::fs::read_to_string("emoji-data.txt").unwrap();
    for line in sequences.lines().map(|l| l.trim()).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let line2 = line.split_once('#').unwrap().1;
        let (rest, name) = line2.split_once(')').expect(line2);
        for (n, e) in name.split("..").zip(rest.split("..")) {
            if n.contains("reserved") {
                continue;
            }
            let e = e.trim_matches(|c: char| (c as u32) < 128);
            let e = e.chars().filter(|&c| {
                !matches!(c as u32, 127995..=127999 | 65039)
            }).collect::<Vec<_>>();
            if e.len() != 1 {
                continue;
            }
            let ch = e[0] as usize;
            if ch == 8205 || ch == 8419 || ch == 917536 || ch == 917631{
                continue;
            }
            if all_codepoints.len() <= ch {
                all_codepoints.resize(ch as usize+1, UNKNOWN_CHAR);
            }
            if "" == all_codepoints[ch] || "[?]" == all_codepoints[ch] || UNKNOWN_CHAR == all_codepoints[ch] {
                let new_name = format!("{} ", n.trim()
                    .to_lowercase()
                    .replace('_', " ")
                    .trim_start_matches("lower right ")
                    .trim_start_matches("upper right ")
                    .trim_start_matches("trigram for ")
                    .trim_start_matches("reversed ")
                    .trim_start_matches("rotated ")
                    .trim_start_matches("heavy ")
                    .trim_end_matches(" symbol")
                    .trim_end_matches(" suit")
                    .trim_end_matches(" bullet")
                    .chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>());
                all_codepoints[ch] = Box::leak(new_name.into_boxed_str());
            }
        }
    }

    for (ch, replacement) in all_codepoints.iter_mut().enumerate() {
        let Ok(ch) = (ch as u32).try_into() else {
            continue;
        };
        let cat = GeneralCategory::of(ch);
        use GeneralCategory::*;
        if cat == Control {
            *replacement = "";
        }

        if *replacement == UNKNOWN_CHAR {
            match cat {
                SpacingMark | SpaceSeparator => {
                    *replacement = " ";
                },
                FinalPunctuation => {
                    *replacement = ".";
                },
                OtherPunctuation => {
                    *replacement = "_";
                },
                DashPunctuation => {
                    *replacement = "-";
                },
                NonspacingMark | EnclosingMark | ModifierSymbol | Format | Surrogate => {
                    *replacement = "";
                },
                _ => {},
            }
        }
    }

    // phrases need to end with a space
    for bad in all_codepoints.iter_mut().filter(|c| c.contains(' ') && c.starts_with(|c: char| c.is_ascii_alphabetic()) && !c.ends_with(' ')) {
        *bad = Box::leak(format!("{} ", bad).into_boxed_str());
    }

    while all_codepoints.last().copied() == Some(UNKNOWN_CHAR) {
        all_codepoints.pop();
    }

    println!("Got {} codepoints to {} chars",
        all_codepoints.iter().filter(|&&c| c != "" && c != UNKNOWN_CHAR).count(),
        all_codepoints.iter().filter(|&&c| c != "" && c != UNKNOWN_CHAR).map(|s| s.len()).sum::<usize>(),
    );

    // find most popular replacements
    let mut popularity = HashMap::<&str, (isize, usize)>::new();
    for (n, replacement) in all_codepoints.iter()
        .filter(|&&r| r.len()>2 && r != UNKNOWN_CHAR) // 0..=2 len gets special treatment
        .enumerate() {
        popularity.entry(replacement).or_insert((1,n)).0 -= 1;
    }

    // and sort them by most popular first
    // most popular first mean small numbers will be most frequently used
    // which is good for compression
    // then by longest first, so that we can reuse common prefixes
    // then roughly group by similarity (original order + alpha)
    let mut by_pop = popularity.iter()
        .map(|(&rep,&(pop, n))| (rep.chars().any(|c| c.is_ascii_uppercase() || !c.is_ascii_alphabetic()), !rep.chars().any(|c| c == ' '),  pop == 0,pop/4,rep.chars().any(|c| c.is_ascii_uppercase()),!rep.len(),n/4, rep))
        .collect::<Vec<_>>();
    by_pop.sort();

    // find redundant replacements that are prefixes/suffixes of existing ones
    // so if "abc" is stored, "ab" is redundant.
    // I should use a suffix tree but I'm lazy and Rust is fast
    let mut longer = HashMap::<&str, &str>::new();
    for &(..,replacement) in by_pop.iter() {
        if longer.get(replacement).is_none() {
            let mut r = replacement;
            while r.len() > 2 {
                let mut p = r;
                while p.len() > 2 {
                    longer.entry(p).and_modify(|old| {
                        if old.len() < replacement.len() {
                            *old = replacement;
                        }
                    }).or_insert(replacement);
                    p = &p[1..];
                }
                r = &r[0..r.len()-1];
            }
        }
    }

    // make first word overlap with the last word
    let mut by_pop = by_pop.into_iter().enumerate().map(|(i, (..,w))| {
        (i*2, longer.get(w).copied().unwrap_or(w))
    }).collect::<Vec<_>>();

    let mut last_word = by_pop.iter().rev()
        .filter_map(|&(i, replacement)| {
        Some((replacement.trim().rsplit_once(' ')?.1, (i, replacement)))
    }).collect::<HashMap<_,_>>();

    for (i, replacement) in by_pop.iter_mut() {
        let Some((first_word, _)) = replacement.trim().split_once(' ') else { continue; };
        if let Some((matched, _)) = last_word.remove(first_word) {
            *i = matched+1; // makes them adjacent in the next loop
        }
    }
    by_pop.sort_by_key(|a| a.0);

    // store each longest replacement, saving its position
    let mut mapping = String::with_capacity(60_000);
    let mut index = HashMap::<&str, usize>::new();
    'words: for (_, replacement) in by_pop {
        let replacement = longer.get(replacement).copied().expect("known prefix");
        if index.get(replacement).is_none() {
            // there's a chance two adjacent replacements form a third
            // so "ab", "cd" is useful for "bc"
            if let Some(pos) = mapping.find(replacement) {
                index.insert(replacement, pos);
            } else {
                for n in (1..replacement.len().min(mapping.len())).rev() {
                    if replacement.starts_with(&mapping[mapping.len() - n..]) {
                        mapping.push_str(&replacement[n..]);
                        index.insert(replacement, mapping.len() - n);
                        continue 'words;
                    }
                }
                index.insert(replacement, mapping.len());
                mapping.push_str(replacement);
            }
        }
    }

    // Now write pointers to the mapping string
    // each is position (2 bytes) + length (1 byte)
    let mut pointers = Vec::with_capacity(all_codepoints.len());
    assert!(mapping.len() < u32::max_value() as usize);
    for &replacement in all_codepoints.iter() {
        let pos = match replacement.len() {
            _ if replacement == UNKNOWN_CHAR => {
                0xFFFF // intentionally invalid len will be caught later
            },
            0 => 0,
            1 => {
                let c = replacement.chars().next().unwrap() as usize;
                assert!(c < 128);
                c
            },
            2 => {
                let mut ch = replacement.chars();
                let c1 = ch.next().unwrap() as usize;
                let c2 = ch.next().unwrap() as usize;
                assert!(c1 < 128);
                assert!(c2 < 128);
                c1 | (c2 << 8)
            },
            len => {
                let off = mapping.find(replacement).expect("in index");
                assert_eq!(&mapping[off..off+len], replacement);
                off
            },
        };
        pointers.push((pos & 0xFF) as u8);
        pointers.push((pos >> 8) as u8);
        pointers.push(if pos == 0xFFFF {0xFF} else {replacement.len() as u8});
    }

    let mut f = File::create("../src/pointers.bin").unwrap();
    f.write_all(&pointers).unwrap();
    let mut f = File::create("../src/mapping.txt").unwrap();
    f.write_all(mapping.as_bytes()).unwrap();
}
