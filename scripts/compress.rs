//! Takes data.rs and makes pointers.bin & mapping.txt data files

mod data;
use data::MAPPING;
use std::collections::HashMap;

use std::fs::File;
use std::io::Write;

fn main() {
    // find most popular replacements
    let mut popularity = HashMap::<&str, (isize, usize)>::new();
    for (n, replacement) in MAPPING.iter()
        .filter(|r|r.len()>2) // 0..=2 len gets special treatment
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
    for replacement in MAPPING.iter() {
        let pos = match replacement.len() {
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
