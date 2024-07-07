use gumdrop::Options;
use indexmap::IndexMap;
use std::fs::File;
use std::process::Command;
use std::io::prelude::*;
mod rng;
mod shuffled;
pub fn write_pre_shuffle() -> std::io::Result<()> {
    let mut file = File::create("src/shuffled.rs")?;
    // one roll index
    let mut shuffled: [u16; 0x10000] = [0; 0x10000];
    for i in 0..0x10000 {
        shuffled[i] = rng::shuffle_rng(i as u16);
        // dbg!(shuffled[i]);
    }

    // create shortcut arrays using one roll index
    let mut by_repeats: [[u16; 0x10000]; 0x10] = [[0; 0x10000]; 0x10];
    for index in 0..16 {
        let repeats = index + 3;
        for i in 0..=0xFFFF {
            let mut s = i;
            for _ in 0..repeats {
                s = rng::shuffle_rng(i as u16)
            }
            by_repeats[index][i as usize] = s;
            // dbg!(by_repeats[index][i as usize]);
        }
    }
    file.write_all(b"pub static BY_REPEATS: [[u16; 0x10000]; 0x10] = [\n")?;
    for index in 0..16 {
        file.write_all(b"    [\n")?;
        for seed_idx in 0..=0xFFFF {
            let seed = by_repeats[index][seed_idx];
            file.write_all(format!("        0x{seed:04X},\n").as_bytes())?;
        }
        file.write_all(b"    ],\n")?;
    }
    file.write_all(b"];")?;

    let _ = Command::new("rustfmt").arg("src/shuffled.rs").output().expect("");
    Ok(())
}

fn crunch_seed(
    seed1: u8,
    seed2: u8,
    seed3: u8,
    known_seeds: &mut IndexMap<(u8, u8, u8), (u32, u32, (u8, u8, u8, u8))>,
    known_loops: &mut IndexMap<(u8, u8, u8, u8), IndexMap<(u8, u8, u8, u8), (u8, u8, u8, u8)>>,
    orientation_ids: &[u8; 0x7],
) {
    let og = (seed1, seed2 & 0xFE, seed3 & 0xF7);
    if known_seeds.contains_key(&og) {
        return;
    }
    let mut steps = 0;
    let mut path: IndexMap<(u8, u8, u8, u8), (u8, u8, u8, u8)> = IndexMap::new();
    let mut repeats = 18;
    if (seed3 >> 4) > 0 {
        repeats = (seed3 >> 4) + 2;
    }
    let mut spawn_id: u8 = 0;
    let mut s1 = seed1;
    let mut s2 = seed2;
    let mut s3 = seed3;
    loop {
        let original = (s1, s2, s3, spawn_id);
        // dbg!(original);
        for (loop_id, loop_map) in known_loops.into_iter() {
            if loop_map.contains_key(&original.clone()) {
                let index = loop_map
                    .into_iter()
                    .position(|(&l, _)| l == original)
                    .unwrap();
                // println!("New seed found: {:#?}", loop_id);
                known_seeds.insert(og, (steps as u32, index as u32, *loop_id));
                return;
            }
        }
        // dbg!(s1, s2, s3, spawn_id);
        let new = rng::get_next_piece(s1, s2, s3, spawn_id, repeats, &orientation_ids);
        (s1, s2, s3, spawn_id) = new;
        // dbg!(s1, s2, s3, spawn_id);
        let new = (s1, s2, s3, spawn_id);
        if path.contains_key(&new) {
            // dbg!(path.clone());
            let loop_start = path
                .clone()
                .into_iter()
                .position(|(l, _)| l == new)
                .unwrap();
            let mut new_loop = IndexMap::new();
            // dbg!(loop_start);
            for (k, v) in path.clone().into_iter() {
                let (i, _, _) = path.clone().get_full(&k).unwrap();
                // dbg!((i,(k,v)));
                if i < loop_start {
                    continue;
                }
                // dbg!(i);
                new_loop.insert(k, v);
            }
            // dbg!(new_loop.clone());
            known_loops.insert(new, new_loop);
            known_seeds.insert(og, (steps as u32, 0, new));
            return;
        }
        // dbg!(path.clone());
        steps += 32;
        path.insert(original, new);
    }
}

fn parse_hex(s: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(s, 16)
}

#[derive(Debug, Options)]
struct TestOptions {
    help: bool,
    #[options(help = "make pre shuffle table")]
    make_table: bool,
    foo: bool,
}

fn main() {
    let options = TestOptions::parse_args_default_or_exit();
    if options.make_table {
        println!("Making new shuffle table");
        let _ = write_pre_shuffle();

        return;
    }
    let orientation_ids: [u8; 0x7] = [0x02, 0x07, 0x08, 0x0A, 0x0B, 0x0E, 0x12];
    let mut known_seeds: IndexMap<(u8, u8, u8), (u32, u32, (u8, u8, u8, u8))> = IndexMap::new();
    let mut known_loops: IndexMap<(u8, u8, u8, u8), IndexMap<(u8, u8, u8, u8), (u8, u8, u8, u8)>> =
        IndexMap::new();


    println!("Hello, world!");
    for x in 0..=0xF {
        println!("x is {}", x);
        for y in 0..=0xFF {
            for z in 0..=0xFF {
                crunch_seed(
                    x,
                    y,
                    z,
                    &mut known_seeds,
                    &mut known_loops,
                    &orientation_ids,
                );
            }
        }
        // what to do with known_seeds and known_loops?
    }
    println!(
        "aasdfasdf {} {}",
        known_seeds.keys().len(),
        known_loops.keys().len()
    );

    for (loop_id, loop_map) in known_loops.into_iter() {
        println!(
            "Loop id {} {} {} {} - {}",
            loop_id.0,
            loop_id.1,
            loop_id.2,
            loop_id.3,
            loop_map.keys().len(),
        );
    }
}
