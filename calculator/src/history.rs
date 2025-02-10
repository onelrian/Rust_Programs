use crate::Calculation;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

pub fn load_history() -> io::Result<Vec<Calculation>> {
    let history_file = "history.txt";
    if !std::path::Path::new(history_file).exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(history_file)?;
    let history: Vec<Calculation> = data
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                let operator = parts[0].to_string();
                let numbers: Vec<f64> = parts[1]
                    .split(',')
                    .map(|s| s.parse().unwrap_or(0.0))
                    .collect();
                let result = parts[2].parse().unwrap_or(0.0);
                Some(Calculation {
                    operator,
                    numbers,
                    result,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(history)
}

pub fn save_history(history: &[Calculation]) -> io::Result<()> {
    let history_file = "history.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(history_file)?;

    for calc in history {
        let numbers_str = calc
            .numbers
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",");
        writeln!(file, "{}|{}|{}", calc.operator, numbers_str, calc.result)?;
    }

    Ok(())
}
