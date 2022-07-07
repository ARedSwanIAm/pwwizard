pub use use clap::Parser;
pub use rand::{self, seq::SliceRandom, Rng};

#[derive(Parser, Debug)]
#[clap()]
pub struct Args {
    // length of the password
    #[clap(short, long)]
    pub length: u8,

    #[clap(short, long)]
    pub special: bool,
}

pub fn get_cmdla() -> Args {
    Args::parse()
}

pub fn pick_chars(
    pass_len: u8,
    chars_str: &'static str,
    mut chars_num: u8,
) -> (u8, Vec<&'static str>) {
    /// chars_num is the length that the component (exemple specail
    /// characters) will be
    // gen random chars and don't exceed pass len
    let mut password_str: Vec<&str> = vec![];

    let mut rng = rand::thread_rng();
    loop {
        if chars_num == 0 {
            break;
        }
        let picked_chars_start = rng.gen_range(0..chars_str.len());
        password_str.push(
            chars_str
                .get(picked_chars_start..(picked_chars_start + 1))
                .unwrap(),
        ); // !! iterate vec and select get multiple
        chars_num -= 1;
    }
    (pass_len - pass_len / 4_u8, password_str) //LOGIC?
}

/// randomize the order of all character in the password
pub fn randomise_password<'a>(password: Vec<&'a str>) -> Vec<&'a str> {
    let mut rng = rand::thread_rng();
    let password: Vec<&'a str> = password
        .choose_multiple(&mut rng, password.len())
        .cloned()
        .collect();
    password
}

// TEST --------------------------------------------------------
/// Unit test
#[cfg(test)]
mod UnitTest {
    use super::*;
    #[test]
    //#[ignore]
    /// testing the pick_chars function
    pub fn password_chars_type_division() {
        let password_len = 12;
        let upper_case_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let chars_num = password_len / 4_u8;

        let (querter_password, _) = pick_chars(password_len, upper_case_chars, chars_num);
        assert_eq!(querter_password, 3 * password_len / 4);
    }

    /// testing the randomise_pass function
    #[test]
    pub fn randomise_pass() {
        let password = ["!", "a", "e", "2", "@", "V", "r", "7"];
        let randomised_password = randomise_password(password.into());
        println!("{:?}", &randomised_password);
        assert_ne!(randomised_password, password);
    }
}
