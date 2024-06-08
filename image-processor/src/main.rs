use std::env;
use std::process;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() || args.len() > 3 {
        eprintln!("Incorrect use, please try again");
        process::exit(1);
    }
    let query = &args[1];
    let file_path = &args[2];

    print!("Applying {}", query);
    println!(" to {}", file_path);

    let width: i32;
    let height: i32;
    let channels: i32;

    

}
