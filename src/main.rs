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
    if options.stats {
        println!("Length is {}", options.length);
    }
    dbg!(options.seed);
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
    }
    if !options.stats {
        return;
    };

    let mut most_longbars: u16 = 0;
    let mut least_longbars: u16 = length as u16;
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
                let sequence = rng::crunch_seed(x, y, z, &shuffled, &by_repeats, length);
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
