use gumdrop::Options;
mod rng;

const SEED_LIMIT: u16 = 850;

fn crunch_seed(
    seed1: u8,
    seed2: u8,
    seed3: u8,
    shuffled: &Vec<u16>,
    by_repeats: &Vec<Vec<u16>>,
) -> Vec<u8> {
    let mut sequence: Vec<u8> = Vec::new();
    let repeat_nybble = seed3 >> 4;
    let mut spawn_id: u8 = 0;
    let mut s1 = seed1;
    let mut s2 = seed2;
    let mut s3 = seed3;
    for _ in 0..SEED_LIMIT {
        // dbg!(s1, s2, s3, spawn_id);
        (_, s1, s2, s3, spawn_id) =
            rng::get_next_piece(repeat_nybble, s1, s2, s3, spawn_id, &shuffled, &by_repeats);
        sequence.push(spawn_id);
    }
    return sequence;
}

#[derive(Debug, Options)]
struct TestOptions {
    help: bool,
    #[options(help = "make pre shuffle table")]
    make_table: bool,
    foo: bool,
}

fn main() {
    // let options = TestOptions::parse_args_default_or_exit();
    // if options.make_table {
    //     println!("Making new shuffle table");
    //     let _ = write_pre_shuffle();

    //     return;
    // }
    let (shuffled, by_repeats) = rng::get_pre_shuffle();
    let mut most_longbars: u16 = 0;
    let mut least_longbars: u16 = SEED_LIMIT;
    let mut most_squares: u16 = 0;

    let mut ml_seed: i32 = 0;
    let mut ll_seed: i32 = 0;
    let mut ms_seed: i32 = 0;

    println!("Hello, world!");
    for x in 0x00..=0xFF {
        println!("x is {}", x);
        for y in 0x00..=0xFF {
            // println!("y is {}", y);
            for z in 0x00..=0xFF {
                if z & 0x08 == 0x08 || y & 0x01 == 0x01 {
                    // println!("Skipping duplicate seed {:02X}{:02X}{:02X}", x,y,z);
                    continue;
                }
                let sequence = crunch_seed(x, y, z, &shuffled, &by_repeats);
                let longbars = sequence.clone().into_iter().filter(|s| *s == 18).count();
                let squares = sequence.clone().iter().filter(|&s| *s == 10).count();

                if longbars as u16 > most_longbars {
                    ml_seed = ((x as i32) << 16 | (y as i32) << 8 | z as i32) as i32;
                    most_longbars = longbars as u16;
                }
                if (longbars as u16) < least_longbars {
                    ll_seed = ((x as i32) << 16 | (y as i32) << 8 | z as i32) as i32;
                    least_longbars = longbars as u16;
                }
                if squares as u16 > most_squares {
                    ms_seed = ((x as i32) << 16 | (y as i32) << 8 | z as i32) as i32;
                    most_squares = squares as u16;
                }
                // println!("Sequence: {:#?}", sequence);
            }
        }
    }
    println!("Most squares: {:06X} {}", ms_seed, most_squares);
    println!("Most longbars: {:06X} {}", ml_seed, most_longbars);
    println!("Least longbars: {:06X} {}", ll_seed, least_longbars);
}
