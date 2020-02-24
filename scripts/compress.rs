//! Takes data.rs and makes pointers.bin & mapping.txt data files


#[macro_use] extern crate serde_derive;

mod data;
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
    let mut all_codepoints: Vec<_> = MAPPING.iter().map(|&ch| {
        if ch != "[?] " && ch != "[?]" {ch} else {UNKNOWN_CHAR} // old data marks unknown as "[?]"
    }).collect();
    for &(ch, ref name) in gemoji.iter().chain(emoji1.iter()).chain(emoji2.iter()) {
        while all_codepoints.len() <= ch {
            all_codepoints.push("");
        }
        if "" == all_codepoints[ch] || "[?]" == all_codepoints[ch] || UNKNOWN_CHAR == all_codepoints[ch] || name.len() < all_codepoints[ch].len() {
            all_codepoints[ch] = name;
        }
    }

    // find most popular replacements
    let mut popularity = HashMap::<&str, (isize, usize)>::new();
    for (n, replacement) in all_codepoints.iter()
        .filter(|&&r| r.len()>2 && r != UNKNOWN_CHAR) // 0..=2 len gets special treatment
        .enumerate() {
        popularity.entry(replacement).or_insert((0,n)).0 -= 1;
    }
    // and sort them by most popular first
    // most popular first mean small numbers will be most frequently used
    // which is good for compression
    // then by longest first, so that we can reuse common prefixes
    // then roughly group by similarity (original order + alpha)
    let mut by_pop = popularity.iter()
        .map(|(&rep,&(pop, n))| (pop/4,-(rep.len() as isize),n/4,rep))
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
                    longer.insert(p, replacement);
                    p = &p[1..];
                }
                r = &r[0..r.len()-1];
            }
        }
    }

    // store each longest replacement, saving its position
    let mut mapping = String::new();
    let mut index = HashMap::<&str, usize>::new();
    for (..,replacement) in by_pop {
        let replacement = *longer.get(replacement).expect("known prefix");
        if index.get(replacement).is_none() {
            // there's a chance two adjacent replacements form a third
            // so "ab", "cd" is useful for "bc"
            if let Some(pos) = mapping.find(replacement) {
                index.insert(replacement, pos);
            } else {
                index.insert(replacement, mapping.len());
                mapping.push_str(replacement);
            }
        }
    }

    // Now write pointers to the mapping string
    // each is position (2 bytes) + length (1 byte)
    let mut pointers = Vec::new();
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
            _ => {
                let l = *longer.get(replacement).expect("known prefix");
                *index.get(l).expect("in index")
            },
        };
        pointers.push((pos & 0xFF) as u8);
        pointers.push((pos >> 8) as u8);
        pointers.push(replacement.len() as u8);
    }

    let mut f = File::create("../src/pointers.bin").unwrap();
    f.write_all(&pointers).unwrap();
    let mut f = File::create("../src/mapping.txt").unwrap();
    f.write_all(mapping.as_bytes()).unwrap();
}
