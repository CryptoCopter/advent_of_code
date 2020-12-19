mod passports;

fn main() {
    let required: Vec<String> = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
    ];
    let batch = passports::load_batch("input/passports.txt");
    println!(
        "Number of valid passports: {}",
        passports::count_valid(&batch, &required)
    );
}
