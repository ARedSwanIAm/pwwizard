use divrem::DivCeil;
use rand::{self, Rng};

fn main() {
    let mut yn: &str = "";
    if pwwizard::get_cmdla().special {
        yn = "";
    } else {
        yn = "no";
    }

    let lower_case_chars = "abcdefghijklmnopqrstuvwxyz";
    let upper_case_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let special_chars = "!@#$%^&*()_+{}|:',./<>?~`[];";
    let numbers = "1234567890";
    let mut password = vec![];

    println!(
        "you want the pass word to be {:} chars long with {:} spacial characters",
        pwwizard::get_cmdla().length,
        &yn
    );

    let mut rng = rand::thread_rng();

    let pass_len = pwwizard::get_cmdla().length;
    let chars_num = pass_len.div_ceil(4_u8);
    let (reduced_passlen, mut included_nums) =
        pwwizard::pick_chars(pass_len.clone(), numbers, chars_num);
    let (reduced_passlen, mut included_lowers) =
        pwwizard::pick_chars(reduced_passlen, lower_case_chars, chars_num);
    let (mut reduced_passlen, mut included_uppers) =
        pwwizard::pick_chars(reduced_passlen, upper_case_chars, chars_num);
    password.append(&mut included_uppers);
    //println!("len: {}, password: {:?}", password.len(), &password); //DEBUG
    password.append(&mut included_lowers);
    password.append(&mut included_nums);
    //println!("len: {}, password: {:?}", password.len(), &password); //DEBUG

    if pwwizard::get_cmdla().special {
        let (_password_reduced, mut included_special) =
            pwwizard::pick_chars(reduced_passlen, special_chars, chars_num);
        password.append(&mut included_special);
        //println!("len: {}, password: {:?}", password.len(), &password); //DEBUG
    } else {
        // incase spacial character was not included, fill in the remaining space
        while reduced_passlen > password.len() as u8 {
            let random_index = rng.gen_range(0..password.len());
            password.push(password.get(random_index).unwrap_or(&"X"));
            reduced_passlen -= 1;
        }
    }

    let password = pwwizard::randomise_password(password);
    println!(
        "Here is your password: {}",
        password.into_iter().collect::<String>()
    )
}
