const ORIENTATION_IDS: [u8; 0x7] = [0x02, 0x07, 0x08, 0x0A, 0x0B, 0x0E, 0x12];
pub fn add_two(num: u16) -> u16 {
    return num + num;
}
pub fn get_pre_shuffle() -> (Vec<u16>, Vec<Vec<u16>>) {
    let mut shuffled: Vec<u16> = vec![0; 0x10000];
    for i in 0..0x10000 {
        shuffled[i] = shuffle_rng(i as u16);
        // dbg!(shuffled[i]);
    }

    // create shortcut arrays using one roll index
    let mut by_repeats: Vec<Vec<u16>> = vec![vec![0; 0x10000]; 0x10];
    for index in 0..0x10 {
        let mut repeats = 0x10;
        if index != 0 {
            repeats = index;
        }
        repeats += 3;
        for i in 0..=0xFFFF {
            let mut s = i;
            for _ in 0..repeats {
                if index == 1 && i == 0x1111 {
                    // dbg!(repeats);
                    println!("{:04X}", s);
                }
                s = shuffle_rng(s as u16)
            }
            if index == 1 && i == 0x1111 {
                // dbg!(repeats);
                println!("{:04X}", s);
            }
            by_repeats[index][i as usize] = s;
            // dbg!(by_repeats[index][i as usize]);
        }
    }
    return (shuffled, by_repeats);
}
pub fn shuffle_rng(rng: u16) -> u16 {
    add_two(2);
    let rng_hi = rng >> 8;
    let rng_lo = rng & 0xFF;
    let newbit = ((rng_hi ^ rng_lo) & 2) << 6;
    let new_hi = newbit | rng_hi >> 1;
    let new_lo = ((rng_hi & 1) << 7) | (rng_lo >> 1);
    return new_hi << 8 | new_lo;
}

pub fn get_next_piece(
    repeat_nybble: u8,
    seed1: u8,
    seed2: u8,
    seed3: u8,
    spawn_id: u8,
    shuffled: &Vec<u16>,
    by_repeats: &Vec<Vec<u16>>,
) -> (u8, u8, u8, u8, u8) {
    // increment and rollover 3rd byte
    // dbg!(seed1);
    // dbg!(seed2);
    // dbg!(seed3);
    // dbg!(spawn_id);
    // dbg!(repeats);
    let s3 = ((seed3 as u16 + 1) & 0xFF) as u8;
    // dbg!(s3);
    let rol_idx = ((seed1 as u16) << 8 | seed2 as u16) as usize;
    // dbg!(rpt_idx);
    // dbg!(rol_idx);
    let roll = by_repeats[repeat_nybble as usize][rol_idx];
    // println!("roll={:04X}", roll);
    // dbg!(shuffled::BY_REPEATS[rpt_idx][rol_idx]);
    // dbg!(roll);
    let mut s1 = (roll >> 8) as u8;
    // dbg!(s1);
    let mut s2 = (roll & 0xFF) as u8;
    // dbg!(s2);
    let mut result = ((s1 as u16 + s3 as u16) & 0x7) as u8;
    // dbg!(result);
    if result == 7 || ORIENTATION_IDS[result as usize] == spawn_id {
        let reroll = shuffled[roll as usize];
        s1 = (reroll >> 8) as u8;
        s2 = (reroll & 0xFF) as u8;
        result = (((s1 & 7) + spawn_id) & 0xFF) % 7;
    }
    let new_id = ORIENTATION_IDS[result as usize];

    // clear out bits that don't do anything
    let new = (repeat_nybble, s1, s2 & 0xFE, s3 & 0x7, new_id);
    // dbg!(new);
    return new;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn test_lfsr() {
        assert_eq!(0x84EF, shuffle_rng(0x09DF));
        assert_eq!(0x5D43, shuffle_rng(0xBA87));
        assert_eq!(0x0420, shuffle_rng(0x0840));
        assert_eq!(0x7AEA, shuffle_rng(0xF5D4));
        assert_eq!(0x0C10, shuffle_rng(0x1820));
        assert_eq!(0x0000, shuffle_rng(0x0000));
    }
    #[test]
    fn test_poop() {
        let (shuffled, by_repeats) = get_pre_shuffle();
        assert_eq!(0x84EF, shuffled[0x09DF]);
        assert_eq!(0x84EF, shuffled[0x09DF]);
        assert_eq!(0x5D43, shuffled[0xBA87]);
        assert_eq!(0x0420, shuffled[0x0840]);
        assert_eq!(0x7AEA, shuffled[0xF5D4]);
        assert_eq!(0x0C10, shuffled[0x1820]);
        assert_eq!(0x0000, shuffled[0x0000]);

        assert_eq!(0x0111, by_repeats[0x1][0x1111]);
    }
    #[test]
    fn test_next_piece_seed() {
        let (shuffled, by_repeats) = get_pre_shuffle();
        // repeat nybble, set_seed+0, set_seed+1, set_seed+2, spawn_id
        assert_eq!(
            (0x01, 0x01, 0x10, 0x02, 0x0A),
            get_next_piece(0x01, 0x11, 0x11, 0x11, 0x00, &shuffled, &by_repeats)
        );
    }
}
