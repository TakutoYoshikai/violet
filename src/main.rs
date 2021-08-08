use std::fs;
use std::env;
use std::io::Read;
use std::fs::File;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::seq::SliceRandom;
use chrono::{DateTime};

type AesCbc = Cbc<Aes256, Pkcs7>;
use chrono::prelude::*;
use rsntp::SntpClient;
const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

const MESSAGES: &'static [&'static str] = &[];
const START_TIME: &str = "";
const KEY: &str = "";

fn violet<'a>(messages: Vec<String>, start_date: DateTime<Local>) -> Option<String> {
    let client = SntpClient::new();
    let result = client.synchronize("time.google.com").unwrap();
    let now: DateTime<Local> = DateTime::from(result.datetime());
    let year = start_date.year();
    let mut next_year: i32 = year as i32;
    for i in 0..messages.len() {
        next_year += 1;
        let dt1 = start_date.with_year(next_year).unwrap();
        if now.timestamp() > start_date.timestamp() && now.timestamp() < dt1.timestamp() {
            let decrypted = std::str::from_utf8(&decrypt(KEY, &messages[i])).unwrap().to_string();
            return Some(decrypted);
        }
    }
    return None;
}

fn read_file(path: &str) -> Option<Vec<u8>> {
    let mut file = File::open(path).unwrap();
    let metadata = fs::metadata(path).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    match file.read(&mut buffer) {
        Ok(_) => (),
        Err(_) => return None,
    }
    return Some(buffer);
}
fn gen_ascii_chars(size: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR.as_bytes()
            .choose_multiple(&mut rng, size)
            .cloned()
            .collect()
    ).unwrap()
}


fn encrypt(key: &str, data: &[u8]) -> String {
    let iv_str = gen_ascii_chars(16);
    let iv = iv_str.as_bytes();
    let cipher = AesCbc::new_var(key.as_bytes(), iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data);
    let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
    buffer.write_bytes(&ciphertext);
    base64::encode(buffer.to_bytes())
}

fn decrypt(key: &str, data: &str) -> Vec<u8> {
    let bytes = base64::decode(data).unwrap();
    let cipher = AesCbc::new_var(key.as_bytes(), &bytes[0..16]).unwrap();
    cipher.decrypt_vec(&bytes[16..]).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        let dt: NaiveDateTime = NaiveDateTime::parse_from_str(START_TIME, "%Y-%m-%d-%H-%M-%S-%z").unwrap();
        let local: DateTime<Local> = Local.from_local_datetime(&dt).unwrap();
        let messages: Vec<String> = MESSAGES.iter().map(|s| s.to_string()).collect();
        print!("{}", violet(messages, local).unwrap());
        return;
    }
    let target: &str = &args[1];
    let key: &str = &args[2];
    let data = read_file(target);
    print!("{}", encrypt(key, &data.unwrap()));
}
