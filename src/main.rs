extern crate sodiumoxide;

use std::env;
use std::process;
use sodiumoxide::crypto::box_;


pub fn to_hex_string(bytes: Box<[u8]>) -> String {
  bytes.iter()
       .map(|b| format!("{:02x}", b))
       .collect::<Vec<_>>()
       .join("")
}


fn main() {
    // Parse args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <pattern>", &args[0]);
        process::exit(1);
    }

    // Get pattern
    let pattern: String = args[1].to_string();
    println!("Searching for keypair that starts with {}...\n", &pattern);

    // Run
    let mut i = 0;
    loop {
        if i % 1000 == 0 {
            println!("Iteration {}...", i);
        }
        let (pk, sk) = box_::gen_keypair();
        let pk_hex = to_hex_string(Box::new(pk.0));
        if pk_hex.starts_with(&pattern) {
            let sk_hex = to_hex_string(Box::new(sk.0));
            println!("\n==> Public: {}", pk_hex);
            println!("==> Secret: {}", sk_hex);
            println!("\nFound key after {} iterations.", i);
            break;
        }
        i += 1;
    }
}
