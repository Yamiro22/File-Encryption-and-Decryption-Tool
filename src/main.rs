use std::fs;
use std::env;
use openssl::symm::{encrypt, decrypt, Cipher};
use openssl::hash::{hash, MessageDigest};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <encrypt|decrypt> <key> <file>", args[0]);
        std::process::exit(1);
    }
    let command = &args[1];
    let key = &args[2];
    let filename = &args[3];

    // Hash the key to ensure it is 32 bytes long
    let key_hash = hash(MessageDigest::sha256(), key.as_bytes()).expect("Failed to hash key");
    let key_bytes = key_hash.as_ref(); // This ensures the key is always 32 bytes

    let data = fs::read(filename).expect("Unable to read file");
    let cipher = Cipher::aes_256_cbc();

    let result = match command.as_str() {
        "encrypt" => encrypt(cipher, key_bytes, None, &data)
            .expect("Encryption failed"),
        "decrypt" => decrypt(cipher, key_bytes, None, &data)
            .expect("Decryption failed"),
        _ => {
            eprintln!("Invalid command: {}", command);
            std::process::exit(1);
        }
    };

    fs::write(filename, result).expect("Unable to write file");
}
