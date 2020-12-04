use std::fs;

pub(crate) fn check_passwords(path: &str) -> (u32, u32) {
    let mut valid_sled: u32 = 0;
    let mut valid_toboggan: u32 = 0;

    let passwords = fs::read_to_string(path).unwrap();

    for password in passwords.lines() {
        let parts: Vec<&str> = password.split(" ").collect();

        let policy_parts: Vec<&str> = parts[0].split("-").collect();
        let policy_low = policy_parts[0].parse::<u32>().unwrap();
        let policy_high = policy_parts[1].parse::<u32>().unwrap();

        let letter_blah: Vec<char> = parts[1].chars().collect();
        let important_letter = letter_blah[0];

        let mut occurences: u32 = 0;
        let letters: Vec<char> = parts[2].chars().collect();
        let mut is_valid = false;
        for i in 0..letters.len() {
            let letter = letters[i];
            if letter == important_letter {
                occurences = occurences + 1;
                if (i as u32 == policy_low-1) || (i as u32 == policy_high-1) {
                    is_valid = !is_valid;
                }
            }
        }

        if (policy_low <= occurences) && (occurences <= policy_high) {
            valid_sled = valid_sled + 1;
        }

        if is_valid {
            valid_toboggan = valid_toboggan + 1;
        }
    }

    return (valid_sled, valid_toboggan);
}