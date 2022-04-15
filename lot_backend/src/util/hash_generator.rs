use base64::encode;
use rand::RngCore;
use sha2::{Digest, Sha256};

use chrono::Utc;
use rand::rngs::OsRng;

pub fn generate_hash_with_time(input: &String) -> String {
    let timestamp_nanos = Utc::now().timestamp_nanos(); // e.g. `2014-11-28T12:45:59.324310806Z`
    let hash = Sha256::new()
        .chain_update(input)
        .chain_update(timestamp_nanos.to_string())
        .chain_update(OsRng.next_u64().to_string())
        .finalize();

    encode(&hash)
}
