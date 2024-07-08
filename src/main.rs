use gumdrop::Options;
mod rng;

const DEFAULT_SEED_LENGTH: i32 = 500;

#[derive(Debug, Options)]
struct TestOptions {
    help: bool,

    #[options(help = "sequence length")]
    length: i32,

    #[options(help = "print stats")]
    stats: bool,

    #[options(help = "print sequence")]
    print: bool,

    #[options(help = "specific seed")]
    seed: String,

    foo: bool,
}

fn parse_hex(s: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(s, 16)
}
fn i32_from_bytes(x: u8, y: u8, z: u8) -> i32 {
    return ((x as i32) << 16 | (y as i32) << 8 | z as i32) as i32;
}

fn check_if_most(count: u16, seed: i32, current_most: u16, current_seed: i32) -> (u16, i32) {
    if count > current_most {
        return (count, seed);
    }
    return (current_most, current_seed);
}
fn check_if_least(count: u16, seed: i32, current_least: u16, current_seed: i32) -> (u16, i32) {
    if count < current_least {
        return (count, seed);
    }
    return (current_least, current_seed);
}

fn main() {
    let options = TestOptions::parse_args_default_or_exit();
    let length = if options.length > 0 {
        options.length
    } else {
        DEFAULT_SEED_LENGTH
    };
    let specific_seed = (if options.seed != "" {
        parse_hex(&options.seed)
    } else {
        Ok(0)
    })
    .unwrap();
    let (shuffled, by_repeats) = rng::get_pre_shuffle();

    if specific_seed != 0 {
        let s1 = ((specific_seed >> 16) & 0xFF) as u8;
        let s2 = ((specific_seed >> 8) & 0xFF) as u8;
        let s3 = (specific_seed & 0xFF) as u8;
        if s1 == 0 && s2 & 0xFE == 0 {
            println!("invalid seed {:06X}", specific_seed);
            return;
        };
        let sequence = rng::crunch_seed(s1, s2, s3, &shuffled, &by_repeats, length);
        if options.print {
            println!("{}", rng::get_string_from_sequence(&sequence))
        };
    }

    let mut least_t: u16 = length as u16;
    let mut least_j: u16 = length as u16;
    let mut least_z: u16 = length as u16;
    let mut least_o: u16 = length as u16;
    let mut least_s: u16 = length as u16;
    let mut least_l: u16 = length as u16;
    let mut least_i: u16 = length as u16;

    let mut least_t_seed: i32 = 0;
    let mut least_j_seed: i32 = 0;
    let mut least_z_seed: i32 = 0;
    let mut least_o_seed: i32 = 0;
    let mut least_s_seed: i32 = 0;
    let mut least_l_seed: i32 = 0;
    let mut least_i_seed: i32 = 0;

    let mut most_t: u16 = 0;
    let mut most_j: u16 = 0;
    let mut most_z: u16 = 0;
    let mut most_o: u16 = 0;
    let mut most_s: u16 = 0;
    let mut most_l: u16 = 0;
    let mut most_i: u16 = 0;

    let mut most_t_seed: i32 = 0;
    let mut most_j_seed: i32 = 0;
    let mut most_z_seed: i32 = 0;
    let mut most_o_seed: i32 = 0;
    let mut most_s_seed: i32 = 0;
    let mut most_l_seed: i32 = 0;
    let mut most_i_seed: i32 = 0;

    for x in 0x00..=0xFF {
        for y in 0x00..=0xFF {
            for z in 0x00..=0xFF {
                if x == 0 && y & 0xFE == 0 {
                    // invalid seed
                    continue;
                }
                if z & 0x08 == 0x08 || y & 0x01 == 0x01 {
                    // duplicate seed
                    continue;
                }
                let sequence = rng::crunch_seed(x, y, z, &shuffled, &by_repeats, length);
                if options.print {
                    println!("{}", rng::get_string_from_sequence(&sequence))
                };
                if options.stats {
                    let seed = i32_from_bytes(x, y, z);

                    let tpieces = sequence.clone().iter().filter(|&s| *s == 2).count();
                    let jpieces = sequence.clone().iter().filter(|&s| *s == 7).count();
                    let zpieces = sequence.clone().iter().filter(|&s| *s == 8).count();
                    let opieces = sequence.clone().iter().filter(|&s| *s == 10).count();
                    let spieces = sequence.clone().iter().filter(|&s| *s == 11).count();
                    let lpieces = sequence.clone().iter().filter(|&s| *s == 14).count();
                    let ipieces = sequence.clone().iter().filter(|&s| *s == 18).count();

                    (most_t, most_t_seed) =
                        check_if_most(tpieces as u16, seed, most_t, most_t_seed);
                    (most_j, most_j_seed) =
                        check_if_most(jpieces as u16, seed, most_j, most_j_seed);
                    (most_z, most_z_seed) =
                        check_if_most(zpieces as u16, seed, most_z, most_z_seed);
                    (most_o, most_o_seed) =
                        check_if_most(opieces as u16, seed, most_o, most_o_seed);
                    (most_s, most_s_seed) =
                        check_if_most(spieces as u16, seed, most_s, most_s_seed);
                    (most_l, most_l_seed) =
                        check_if_most(lpieces as u16, seed, most_l, most_l_seed);
                    (most_i, most_i_seed) =
                        check_if_most(ipieces as u16, seed, most_i, most_i_seed);

                    (least_t, least_t_seed) =
                        check_if_least(tpieces as u16, seed, least_t, least_t_seed);
                    (least_j, least_j_seed) =
                        check_if_least(jpieces as u16, seed, least_j, least_j_seed);
                    (least_z, least_z_seed) =
                        check_if_least(zpieces as u16, seed, least_z, least_z_seed);
                    (least_o, least_o_seed) =
                        check_if_least(opieces as u16, seed, least_o, least_o_seed);
                    (least_s, least_s_seed) =
                        check_if_least(spieces as u16, seed, least_s, least_s_seed);
                    (least_l, least_l_seed) =
                        check_if_least(lpieces as u16, seed, least_l, least_l_seed);
                    (least_i, least_i_seed) =
                        check_if_least(ipieces as u16, seed, least_i, least_i_seed);
                };
            }
        }
    }

    if !options.stats {
        return;
    };

    println!("T Max: {:06X} {}", most_t_seed, most_t);
    println!("J Max: {:06X} {}", most_j_seed, most_j);
    println!("Z Max: {:06X} {}", most_z_seed, most_z);
    println!("O Max: {:06X} {}", most_o_seed, most_o);
    println!("S Max: {:06X} {}", most_s_seed, most_s);
    println!("L Max: {:06X} {}", most_l_seed, most_l);
    println!("I Max: {:06X} {}", most_i_seed, most_i);

    println!("T Min: {:06X} {}", least_t_seed, least_t);
    println!("J Min: {:06X} {}", least_j_seed, least_j);
    println!("Z Min: {:06X} {}", least_z_seed, least_z);
    println!("O Min: {:06X} {}", least_o_seed, least_o);
    println!("S Min: {:06X} {}", least_s_seed, least_s);
    println!("L Min: {:06X} {}", least_l_seed, least_l);
    println!("I Min: {:06X} {}", least_i_seed, least_i);
}
