use crate::norm_file::read_normal;
use crate::norm_file::read_rain;
use crate::gz_file::ungz;
use std::env;
use std::path::Path;
use std::fs;
mod norm_file;
mod gz_file;

fn main() {

    let help = "Usage: ncat [OPTION]... [FILE/DIR]...

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
            "-g" => ungz(args_bad[2].to_string()),
            _ => {
                if Path::new(&args_bad[1]).is_dir() {
                    let paths = fs::read_dir(&args_bad[1]).unwrap();
                    for path in paths {
                        println!("{}", path.unwrap().path().display());
                    }
                }else {
                    read_normal(args_bad[1].to_string());
                }
            },
        }
    }else {
        println!("You need to supply a file");
    }

}



//read_normal(args_bad[1].to_string())
