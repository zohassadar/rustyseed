use indexmap::IndexMap;

fn shuffle_rng(rng: u16) -> u16 {
    let rng_hi = rng >> 8;
    let rng_lo = rng & 0xFF;
    let newbit = ((rng_hi ^ rng_lo) & 2) << 6;
    let new_hi = newbit | rng_hi >> 1;
    let new_lo = ((rng_hi & 1) << 7) | (rng_lo >> 1);
    return new_hi << 8 | new_lo;
}

fn crunch_seed(
    seed1: u8,
    seed2: u8,
    seed3: u8,
    known_seeds: &mut IndexMap<(u8, u8, u8), (i32, (u8, u8, u8, u8), u32)>,
    known_loops: &mut IndexMap<(u8, u8, u8, u8), IndexMap<(u8, u8, u8, u8), (u8, u8, u8, u8)>>,
    by_repeats: &[[u16; 0x10000]; 0x10],
) {
    let orientation_ids: [u8; 0x7] = [0x02, 0x07, 0x08, 0x0A, 0x0B, 0x0E, 0x12];

    let og = (seed1, seed2 & 0xFE, seed3 & 0xF7);
    if known_seeds.contains_key(&og) {
        return;
    }
    let mut i = 0;
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
                known_seeds.insert(og, (i, *loop_id, index as u32));
                return;
            }
        }
        // dbg!(s3);
        s3 = ((s3 as u16 + 1) & 0xFF) as u8;
        // dbg!(repeats);
        let seed = by_repeats[(repeats - 3) as usize][((s1 as u16) << 8 | s2 as u16) as usize];
        s1 = (seed << 8) as u8;
        s2 = (seed & 0xFF) as u8;
        let mut result = (s1 + s2) & 0x7;
        if result == 7 || orientation_ids[result as usize] == spawn_id {
            let new_seed =
                by_repeats[(repeats - 3) as usize][((s1 as u16) << 8 | s2 as u16) as usize];
            s1 = (new_seed << 8) as u8;
            s2 = (new_seed & 0xFF) as u8;
            result = (((s1 & 7) + spawn_id) & 0xFF) % 7;
        }
        spawn_id = orientation_ids[result as usize];
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
            known_seeds.insert(og, (i, new, 0));
            return;
        }
        // dbg!(path.clone());
        i += 32;
        path.insert(original, new);
    }
}

fn main() {
    let mut known_seeds: IndexMap<(u8, u8, u8), (i32, (u8, u8, u8, u8), u32)> = IndexMap::new();
    let mut known_loops: IndexMap<(u8, u8, u8, u8), IndexMap<(u8, u8, u8, u8), (u8, u8, u8, u8)>> =
        IndexMap::new();
    let mut shuffled: [u16; 0x10000] = [0; 0x10000];
    for i in 0..0x10000 {
        shuffled[i] = shuffle_rng(i as u16);
    }
    let mut by_repeats: [[u16; 0x10000]; 0x10] = [[0; 0x10000]; 0x10];
    for index in 0..16 {
        let repeats = index + 3;
        for i in 0..=0xFFFF {
            let mut s = i;
            for i in 0..repeats {
                s = shuffle_rng(i as u16)
            }
            by_repeats[index][i as usize] = s;
        }
    }
    println!("Hello, world!");
    for x in 0..=0xFF {
        println!("x is {}", x);
        for y in 0..=0xFF {
            for z in 0..=0xFF {
                crunch_seed(x, y, z, &mut known_seeds, &mut known_loops, &by_repeats);
            }
        }
        // what to do with known_seeds and known_loops?
    }
    println!(
        "aasdfasdf {} {}",
        known_seeds.keys().len(),
        known_loops.keys().len()
    );
}
