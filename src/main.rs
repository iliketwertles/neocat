use rainbow_text::Rainbow;
use std::env;
use std::fs;

fn read_normal(file: String) {
    let data = fs::read_to_string(file).unwrap();
    println!("{}", data)
}

fn read_rain(file: String) {
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

fn main() {

    let help = "Usage: ncat [OPTION]... [FILE]...

options:
-h      shows this menu
-r      reads file and outputs as rainbow";

    let args_bad: Vec<String> = env::args().collect();
    if args_bad.len() > 1 {
        match args_bad[1].as_str() {
            "-r" => {
                if args_bad.len() > 2 {
                    read_rain(args_bad[2].to_string());
                }else {
                    println!("You need to supply a file");
                }
            },
            "-h" => println!("{}", help),
            _ => read_normal(args_bad[1].to_string()),
        }
    }else {
        println!("You need to supply a file");
    }

}
