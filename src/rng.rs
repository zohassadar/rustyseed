const ORIENTATION_IDS: [u8; 0x7] = [0x02, 0x07, 0x08, 0x0A, 0x0B, 0x0E, 0x12];

const PIECE_MAP: [&str; 0x13] = [
    "", "", "T", "", "", "", "", "J", "Z", "", "O", "S", "", "", "L", "", "", "", "I",
];

pub fn get_string_from_sequence(sequence: &Box<[u8]>) -> String {
    return sequence
        .iter()
        .fold("".to_string(), |a, b| a + PIECE_MAP[*b as usize]);
}

pub fn get_pre_shuffle() -> (Vec<u16>, Vec<Vec<u16>>) {
    let mut shuffled: Vec<u16> = vec![0; 0x10000];
    for i in 0..0x10000 {
        shuffled[i] = shuffle_rng(i as u16);
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
                s = shuffle_rng(s as u16)
            }
            by_repeats[index][i as usize] = s;
        }
    }
    return (shuffled, by_repeats);
}

pub fn shuffle_rng(rng: u16) -> u16 {
    return (((rng << 8) ^ rng) & 0x200) << 6 | rng >> 1;
}

pub fn get_next_piece(
    repeat_nybble: u8,
    seed: u16,
    seed3: u8,
    spawn_id: u8,
    shuffled: &Vec<u16>,
    by_repeats: &Vec<Vec<u16>>,
) -> (u8, u16, u8, u8) {
    let s3 = ((seed3 as u16 + 1) & 0xFF) as u8;
    let mut roll = by_repeats[repeat_nybble as usize][seed as usize];
    let mut result = (((roll >> 8) + s3 as u16) & 0x7) as usize;
    if result == 7 || ORIENTATION_IDS[result] == spawn_id {
        roll = shuffled[roll as usize];
        result = (((((roll >> 8) & 7) + spawn_id as u16) & 0xFF) % 7) as usize;
    }
    let new_id = ORIENTATION_IDS[result];
    // clear out bits that don't do anything
    let new = (repeat_nybble, roll & 0xFFFE, s3 & 0x7, new_id);
    return new;
}

pub fn crunch_seed(
    seed: u16,
    seed3: u8,
    shuffled: &Vec<u16>,
    by_repeats: &Vec<Vec<u16>>,
    sequence: &mut Box<[u8]>,
    length: i32,
) {
    let repeat_nybble = seed3 >> 4;
    let mut spawn_id: u8 = 0;
    let mut s = seed;
    let mut s3 = seed3;
    for index in 0..length as usize {
        (_, s, s3, spawn_id) =
            get_next_piece(repeat_nybble, s, s3, spawn_id, &shuffled, &by_repeats);
        sequence[index] = spawn_id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_pre_shuffled() {
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
    fn test_sequences() {
        let (shuffled, by_repeats) = get_pre_shuffle();
        let (s, s3) = (0x1111, 0x11);
        let expected = "OOTJTJOSZTZOSLTOJTLTZZIZIOZIJTOLLSJSJZOITZZSOLISOLLZJJZSIZSISIZSLITITSOOIJZJTLOILSSSSTJIJOJLTJSZTSLJTOSLSOSZILSISTOJSIOITSZZTSSTZJZITZZSZITJTISLOZILITZJOZSTTZIOZTLISITLLSISLITLISOOZTLSISSLTSZSTJZTSJTSJZZILIOSLTSZOZSZOTILOLOZTIZISJTLJLJZIZJTZOJLOSOITITOTOSTLIOZOTJOTSOZIJTIJOTOLITOJOZIZOLTZJOLJISTLJOZJJLITLJSOJTZOILSZSZJOZTSTJISLSITISJTZTSISZSZTSJLSJOTLJTJJLOJLITZJITJOZLTIJSLTZJIOISTZOJLJLZIOTSSLOJZLJLTITSLZLZTLSISZTOTZTJSTSLJTZSLSIZOITLTJOZSZZTIJTJJLZTZLSLTZJZILITSLJSZLLJLSLJTJOOTZSJTLITTOOTJTSLJSILSILITZLZJIZSTOZIOTLTOIJOZTLTZLSTJZIOITIOJZSLSTITZTLSZJSLITJZJTZLSTSJZOJTZJIJOTZOLLIZOTIJSOJOOTLTLZJOOJLTJZSOIZITLIJJSSZSLSIOTOLTJTOTOLISLJSJOTZLITZJLZTLIILTJJISZOSJIZLSTZOITIOTOSZJLOSTIOZSZOSLZJOZTSSIOTSZTTJLSIOSZTLJZOZSSTJLOJZITZJLTSSZIOOSTZTJSTLTLOSTTLSITZTOZLOTLJILTZISZJTLOSJSOJLJLTJLITZJIOZTISZLZJJTSLOSJIJSTSLOJTZLJTSLSOTZOIIZOJISTJOSTLSOSTZIOTOSZJLJZLZJLSZLISTZSITSITZTZOLSSOTITOILIZJZJTISTLSSJLZITZLTZOTJITJOIOTLZJTISLSZISSIOIZJLSISLZSZSITISLTISITJTZJSLZZJLIJLZTIZIJOZIJOITIOJZOOSTOIZOLJSO";
        let mut sequence = vec![0; expected.len() as usize].into_boxed_slice();
        let _ = crunch_seed(s, s3, &shuffled, &by_repeats, &mut sequence, 1000);
        let string = get_string_from_sequence(&sequence);
        assert_eq!(string, expected);

        let (s, s3) = (0xFFFF, 0xFF);
        let expected = "TZTSILSOOJZJZSOZOLILTZSJOIOTLJSTZOJILTSTLJZILTTISJOZJTSLZJSZTSIOISLTSSLJSSTOOISTZJOTOTOZSILISOLSITZIOZLJZOZSZZOLTLLTZIJLZLOZILSTSOIJLTOTZLILOJOTIOSTJLTOSJLSTJLOZLSISOLJZOJILJSZOSTSITOJZJZIOSITZTOJSIJISZLJOZTITZIJOISIJTJSOJZSTZILOJTZISITIOSOTIZOOZJOZJTZZILIOITOTILTJOIJIZOOLZOIZJZZJZISJZSOTZLTJLSIOTIJSZSJSTOSJLSISTZTJOOILOZOIJTOTLTJIOLSTZJSTZOJZJZTOJJILSZTSLIZJTITSZLZSLOTZOTITJLJIJLZSOTLSLZJSTZTTZOITLTJSJOTIOISOSLOZLZJLJZLTSLZTJIZTZOITLJSLIJLTJSJTLSJTJOZZOTSOJOZOIJITZLZIOLOZIZIJTSTIOTJSIOTSOLTJZOLJSOSJSIJIJJTSLOZZLSOTZTSZILSIOZLZZTSJZJTSZSISTSISOIOJSIOOLTOJSOTOSZISSTLZIOZIOIZJZJLZLZTJITLTLJOTIOSLOSOOJSLSJTLTSJZLOJTJZJSOZJSTOTSOTSZTZTSOJZJLJTSTZJOILTLIZTOJTILSTILTIOLTZLLZIJOZJTSILOSILITOJSOJZSTOZLOZOLSJZIJZJIJSZLOSLSLLTTSOZOOTSISISTJISSOLOISOTZOIJIOLTISOOTSJOJJTIOTOILSTLSOJSSOSLISLLSZOOZIOJSOTSIOSTZTITSIZTOJOJOISIOJOZOLITOTLJOZSIZLOTZTJOZJSOTSSTOSTSIJILISIZSZJTIJOTJOLTISZJIOILZSITOIOSJZTZOSTOSSOLIZJSOZLOLTOLOZSJLOJTZILZITLSJOSOZOLSISLZLJSLTISOILOLIJLIZSIZTZTZTJZJSLTLISJSTSJOZOOTJSIOOSTOLT";
        let mut sequence = vec![0; expected.len() as usize].into_boxed_slice();
        let _ = crunch_seed(s, s3, &shuffled, &by_repeats, &mut sequence, 1000);
        let string = get_string_from_sequence(&sequence);
        assert_eq!(string, expected);

        let (s, s3) = (0x8888, 0x88);
        let expected = "JOSTZIOJSTLTTLSOLSZOLISLIOLTSSOTZOZTJJTJZSIJLTSJTZTJLTSJSISTTZZJTJZSTIOZISJJZJSJSJILOJLSIOTJLOSJILIZJSSLJILTOJISIJZLOTSJOIOTTIJTOOISJOZJTJIOLSIOISTJILZILITJSOTOSZSISLITZOIZJTOLOSJIOLILIOLSOZSOIOZTTOOLITZITTJTLOLITSJTOSTIOLTIOTSTZJTOSZTJZJSSZJZLJTZTSIJOIZOJLZJZLIOLTJOIOTZISOJLSSOLIOIZILOTSLZOLTJOSIZIOLZLZOLISOJLTZLSOSZTSZSTOLZLOZSJTTIJSZLZLISZLJLSOJTLOZJISJITSILJLJJTZJZLSLOITSITLJLITITTOLILOISOTISOLTSZIZJIJZJLZOTIJOLTTITIZTSTTSITOTJSJSZZLTISTTOSLILSISZSLJOSJSJITIOISJTZIITSIOTJSIJLTITLOJIOJSOOTLZIJZJOOLLZJSOSTSSZISTZOLOZILTSZOTSLJZSLZOLJZISZOTJJSSILZSLIJIOITOZTOLZSOIZLOZSZJOIJLOJOSTJTLSIZLJZZJTIZOLJTOLSOZTJZSJTZLTOJJLIZTSTTISOSILTLISITJTIZOZSZISIZLJIIJTILZJIOITSOOITZSILJLTOIOOJZSZJLSTSLZOZJTJIZJISJZSOJSJTSJLSLOJZTSOLTZJTIZLTISJZSOJLLJTOSIJSLZIZOSJLJJISJOITOLIOOZSSJLTJOIJOSISOJTIOJTJOSOZJZOLIOIZOJOOZLTLTOSITISJZITIZTOTSOSOLOOTIJSOJSZSZLOZSTSTJLOLTSJLTISOSLOTLJSILTJTITZJZJSOZJTZIOZSTJJSOSTILTOTIZTJTOJZIZJZTOSJLIOISLTIZSLISZSTZSTILTIZJOTJJOJZJOLTSOSLTJZOJLSTZLSSZIJIOSIJLSOLZJTJSZSZJLZTOIOZZ";
        let mut sequence = vec![0; expected.len() as usize].into_boxed_slice();
        let _ = crunch_seed(s, s3, &shuffled, &by_repeats, &mut sequence, 1000);
        let string = get_string_from_sequence(&sequence);
        assert_eq!(string, expected);
    }

    #[test]
    fn test_next_piece_seed() {
        let (shuffled, by_repeats) = get_pre_shuffle();
        // repeat nybble, set_seed+0, set_seed+1, set_seed+2, spawn_id
        assert_eq!(
            (0x01, 0x0110, 0x02, 0x0A),
            get_next_piece(0x01, 0x1111, 0x11, 0x00, &shuffled, &by_repeats)
        );
    }
}
