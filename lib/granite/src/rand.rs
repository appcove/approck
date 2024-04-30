use chrono::Utc;
use rand::Rng;
use sha2::{Digest, Sha256};

pub fn random_hex(length: usize) -> String {
    // Returns a 64 character hex string suitable for session IDs, etc...
    let mut rval = String::new();
    while rval.len() < length {
        let mut hasher = Sha256::new();
        let random_bytes: Vec<u8> = rand::thread_rng().gen::<[u8; 32]>().to_vec();
        let time_bytes = Utc::now().timestamp_micros().to_string().into_bytes();
        hasher.update([random_bytes, time_bytes].concat());
        rval.push_str(&format!("{:x}", hasher.finalize()));
    }
    rval[0..length].to_string()
}

pub fn ts_random_hex(length: usize) -> String {
    // Returns a random hex string prefixed with the date in year month day hour minute second format
    let datetime_prefix = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let prefix_len = datetime_prefix.len();
    datetime_prefix + &random_hex(length - prefix_len)
}
