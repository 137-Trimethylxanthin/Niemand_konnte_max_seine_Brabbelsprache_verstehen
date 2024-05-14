use std::{fs::File, io::{BufRead, BufReader}};
mod grammer;
fn main() {
    let file_path = "src/assembly.txt";
    let file_contents = get_file_contents(file_path);
    let parsed_instructions = parse_instruction(file_contents);
    for i in parsed_instructions {
        println!("{}", i);
    }
    
}

fn get_file_contents(file_path: &str) -> Vec<Vec<String>> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap().split_whitespace().map(|s| s.to_string()).collect()).collect();
    lines
}


fn parse_instruction(vec:Vec<Vec<String>> ) -> Vec<String> {
    let mut result = Vec::new();
    for i in vec {
        let mut temp = String::new();
        for j in i {
            if grammer::BRABBLE.contains_key(&j) {
                temp.push_str(grammer::BRABBLE.get(&j).unwrap());
            } else {
                temp.push_str(&j);
            }
            temp.push_str(" ");
        }
        result.push(temp);
    }
    result
}

