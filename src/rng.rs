use crate::{shuffled};

pub fn add_two(num: u16) -> u16 {
    return num + num;
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
    seed1: u8,
    seed2: u8,
    seed3: u8,
    spawn_id: u8,
    repeats: u8, // 3 to 18
    orientation_ids: &[u8; 0x7],
) -> (u8, u8, u8, u8) {
    // increment and rollover 3rd byte
    let s3 = ((seed3 as u16 + 1) & 0xFF) as u8;

    // dbg!(repeats);
    let roll = shuffled::BY_REPEATS[(repeats - 3) as usize][((seed1 as u16) << 8 | seed2 as u16) as usize];
    let mut s1 = (roll << 8) as u8;
    let mut s2 = (roll & 0xFF) as u8;
    let mut result = (s1 + s2) & 0x7;
    if result == 7 || orientation_ids[result as usize] == spawn_id {
        let reroll = shuffled::BY_REPEATS[(repeats - 3) as usize][((s1 as u16) << 8 | s2 as u16) as usize];
        s1 = (reroll << 8) as u8;
        s2 = (reroll & 0xFF) as u8;
        result = (((s1 & 7) + spawn_id) & 0xFF) % 7;
    }
    let new_id = orientation_ids[result as usize];

    // clear out bits that don't do anything
    let new = (s1, s2 & 0xFE, s3 & 0x7, new_id);
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
}
