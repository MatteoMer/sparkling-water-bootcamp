use std::env;

mod encrypt;
mod decrypt;
mod generate;
mod utils;

use encrypt::encrypt;
use generate::generate;
use decrypt::decrypt;

fn main() {
    let default_e: u64 = 65537;

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Please specify a mode: encrypt, decrypt, or generate.");
        return;
    }
    
    let mode = &args[1];

    match mode.as_str() {
        "encrypt" => {
            let m = args.get(2).unwrap();
            let n = args.get(3).unwrap();
            encrypt(m, default_e, n);
        }
        "decrypt" => {
            let d = args.get(2).unwrap();
            let c = args.get(3).unwrap();
            let n = args.get(4).unwrap();
            decrypt(d, c, n);
        }
        "generate" => {
            generate(default_e);
        }
        _ => {
            println!("Invalid mode. Use 'encrypt', 'decrypt', or 'generate'.");
        }
    }
}
