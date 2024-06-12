use anyhow::{Ok, Result};
use rand::{seq::SliceRandom, thread_rng};

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<String> {
    let mut rng = thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER wont be empty"));
    }

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("UPPER wont be empty"));
    }

    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("UPPER wont be empty"));
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("UPPER wont be empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        let c: &u8 = chars
            .choose(&mut rng)
            .expect("chars wont be empty in this context");
        password.push(*c);
    }
    password.shuffle(&mut rng);
    let passwords = String::from_utf8(password)?;

    Ok(passwords)
}
