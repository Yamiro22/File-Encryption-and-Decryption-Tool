use std::fs;
use std::env;
use openssl::symm::{encrypt, decrypt, Cipher};
use openssl::hash::{hash, MessageDigest};
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use log::error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<OsString> = env::args_os().collect();
    match args.as_slice() {
        [_, command, key, filename] => {
            let command = command.to_str().ok_or("Invalid command")?;
            let key = key.to_str().ok_or("Invalid key")?;
            let filename = filename.to_str().ok_or("Invalid filename")?;

            // Derive the key using PBKDF2 to ensure it is 32 bytes long
            let mut key_bytes = [0u8; 32];
            let salt = [0u8; 16]; // Use a random salt for added security
            let iterations = 10000; // Choose an appropriate number of iterations
            openssl::pkcs5::pbkdf2_hmac(key.as_bytes(), &salt, iterations, MessageDigest::sha256(), &mut key_bytes)?;

            let data = fs::read(filename)?;

            let cipher = Cipher::aes_256_cbc();

            let result = match command {
                "encrypt" => encrypt(cipher, &key_bytes, None, &data)?,
                "decrypt" => decrypt(cipher, &key_bytes, None, &data)?,
                _ => {
                    return Err(format!("Invalid command: {}", command).into());
                }
            };

            let new_filename = format!("{}_new", filename);
            fs::write(new_filename, result)?;
        },
        _ => {
            return Err("Usage: <encrypt|decrypt> <key> <file>".into());
        }
    }
    
    Ok(())
}
