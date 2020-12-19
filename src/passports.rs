use std::collections::HashMap;
use std::fs;

pub(crate) fn load_batch(path: &str) -> Vec<HashMap<String, String>> {
    let mut datasets: Vec<HashMap<String, String>> = Vec::new();

    let data = fs::read_to_string(path).unwrap();
    for block in data.split("\n\n") {
        let mut dataset: HashMap<String, String> = HashMap::new();
        for line in block.split("\n") {
            for item in line.split(" ") {
                let pair: Vec<&str> = item.split(":").collect();
                if pair.len() < 2 {
                    continue;
                }
                dataset.insert(pair[0].to_string(), pair[1].to_string());
            }
        }
        datasets.push(dataset);
    }

    datasets
}

pub(crate) fn count_valid(
    passports: &Vec<HashMap<String, String>>,
    required_fields: &Vec<String>,
) -> u32 {
    let mut valid = 0;

    for passport in passports.iter() {
        let mut is_valid = true;

        for required in required_fields.iter() {
            match passport.get(required) {
                None => is_valid = false,
                _ => continue,
            }
        }

        if is_valid {
            valid += 1;
        }
    }

    valid
}
