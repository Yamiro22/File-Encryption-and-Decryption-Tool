use std::fs;
use std::env;
use openssl::symm::{Crypter, Mode, Cipher};
use openssl::hash::{hash, MessageDigest};
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use log::error;
use rand::Rng;
use openssl::rand::rand_bytes;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if let [_, command, key, filename] = args.as_slice() {
        let command = parse_command(command)?;
        let key_bytes = derive_key(key)?;
        let data = fs::read(filename)?;
        let cipher = Cipher::aes_256_cbc();
        let result = match command {
            Command::Encrypt => encrypt_data(cipher, &key_bytes, &data)?,
            Command::Decrypt => decrypt_data(cipher, &key_bytes, &data)?,
        };
        fs::write(filename, &result)?;
    } else {
        return Err("Usage: <encrypt|decrypt> <key> <file>".into());
    }
    Ok(())
}

fn parse_command(command: &str) -> Result<Command> {
    match command {
        "encrypt" => Ok(Command::Encrypt),
        "decrypt" => Ok(Command::Decrypt),
        _ => Err(format!("Invalid command: {}", command).into()),
    }
}

fn derive_key(key: &str) -> Result<[u8; 32]> {
    let mut key_bytes = [0u8; 32];
    let salt: [u8; 16] = rand::thread_rng().gen();
    let iterations = 100000;
    openssl::pkcs5::pbkdf2_hmac(key.as_bytes(), &salt, iterations, MessageDigest::sha256(), &mut key_bytes)?;
    Ok(key_bytes)
}

fn encrypt_data(cipher: Cipher, key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>> {
    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key, Some(&[0u8; 16]))?;
    encrypter.pad(false);
    let mut result = vec![0; data.len() + cipher.block_size()];
    let count = encrypter.update(data, &mut result)?;
    let final_result = encrypter.finalize(&mut result[count..])?;
    result.truncate(count + final_result);
    Ok(result)
}

fn decrypt_data(cipher: Cipher, key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>> {
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key, Some(&[0u8; 16]))?;
    decrypter.pad(false);
    let mut result = vec![0; data.len() + cipher.block_size()];
    let count = decrypter.update(data, &mut result)?;
    let final_result = decrypter.finalize(&mut result[count..])?;
    result.truncate(count + final_result);
    Ok(result)
}

enum Command {
    Encrypt,
    Decrypt,
}
