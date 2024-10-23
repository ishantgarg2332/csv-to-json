use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

fn csv_to_json(file_path: String) -> () {
    use std::fs;

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let mut keys: Vec<&str> = vec![];

    let mut output: Vec<HashMap<&str, &str>> = vec![];

    for (idx, line) in lines.into_iter().enumerate() {
        if idx == 0 {
            keys = line.split(',').collect();
        } else {
            let temp: Vec<&str> = line.split(',').collect();
            let mut map: HashMap<&str, &str> = HashMap::new();
            for (index, value) in temp.into_iter().enumerate() {
                map.insert(keys[index], value);
            }
            output.push(map);
        }
    }

    println!("{:?}", output);
    // return output;
}

fn main() {
    let mut file_path = String::new();
    print!("Please enter absolute path for your CSV file: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut file_path)
        .expect("Please enter path");
    if let Some('\n') = file_path.chars().next_back() {
        file_path.pop();
    }
    if let Some('\r') = file_path.chars().next_back() {
        file_path.pop();
    }
    csv_to_json(file_path);
}
