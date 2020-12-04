mod passwords;

fn main() {
    let (valid_sled, valid_toboggan) = passwords::check_passwords("input/passwords.txt");
    println!("# of valid passwords (according to sled company): {}", valid_sled);
    println!("# of valid passwords (according to toboggan) company: {}", valid_toboggan);
}
