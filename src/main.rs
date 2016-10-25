extern crate sodiumoxide;

use sodiumoxide::crypto::box_;


pub fn to_hex_string(bytes: Box<[u8]>) -> String {
  bytes.iter()
       .map(|b| format!("{:02x}", b))
       .collect::<Vec<_>>()
       .join("")
}


fn main() {
    println!("Generating keypair...");
    let mut i = 0;
    loop {
        if i % 1000 == 0 {
            println!("Iteration {}...", i);
        }
        let (pk, sk) = box_::gen_keypair();
        let pk_hex = to_hex_string(Box::new(pk.0));
        if pk_hex.starts_with("1337") {
            let sk_hex = to_hex_string(Box::new(sk.0));
            println!("Public: {}", pk_hex);
            println!("Secret: {}", sk_hex);
            println!("Found key after {} iterations.", i);
            break;
        }
        i += 1;
    }
}
