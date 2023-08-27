use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage : ./program <key> <input_file> <output_file>");
        process::exit(1);
    }

    let file = &args[1];

    println!("{}", file);
    
}
