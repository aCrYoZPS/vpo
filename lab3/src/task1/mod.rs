use rand::prelude::*;

pub fn generate_output() -> String {
    let mut exclam_count: u32 = rand::rng().random();
    exclam_count %= 46;
    exclam_count += 5;
    let exclam_string: String = std::iter::repeat('!').take(exclam_count as usize).collect();
    return String::from("Hello, world!\nAnd hi again!\n") + &exclam_string;
}
