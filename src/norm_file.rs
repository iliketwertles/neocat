use std::fs;
use std::env;
use rainbow_text::Rainbow;

pub fn read_normal(file: String) {
    let data = fs::read_to_string(file).unwrap();
    println!("{}", data)
}

pub fn read_rain(file: String) {
    let args_bad: Vec<String> = env::args().collect();
    match args_bad.len() {
        3 => {
            let arg: &str = &file;
            let data = fs::read_to_string(arg).unwrap();
            Rainbow::default().write(data.as_str()).unwrap();
        }
        _ => println!("no.")
    }
}