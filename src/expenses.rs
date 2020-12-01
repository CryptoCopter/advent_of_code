use std::fs;

pub(crate) fn expense_report(path: &str) {
    let expense_file = fs::read_to_string(path)
        .expect("Error reading file");

    let mut expenses = Vec::new();

    for expense_str in expense_file.lines() {
        let expense = expense_str.parse::<u32>().unwrap();
        expenses.push(expense);
    }

    for i in 0..expenses.len()-1 {
        let exp1 = expenses[i];
        for j in i+1..expenses.len()-1 {
            let exp2 = expenses[j];
            if exp1 + exp2 == 2020 {
                println!("The easy answer is {}", exp1*exp2);
            }
            for h in j+1..expenses.len()-1 {
                let exp3 = expenses[h];
                if exp1 + exp2 + exp3 == 2020 {
                    println!("The harder answer is {}", exp1*exp2*exp3);
                }
            }
        }
    }
}