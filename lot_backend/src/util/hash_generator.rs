use rand::RngCore;
use sha2::{Sha256, Sha512, Digest};
use base64::{encode, decode};

use chrono::prelude::*;
use chrono::{DateTime, Utc};
use rand::rngs::OsRng;

pub fn generate_hash_with_time(input : &String) -> String {
    let mut rng = OsRng.next_u64();

    let timestamp_nanos= Utc::now().timestamp_nanos();       // e.g. `2014-11-28T12:45:59.324310806Z`
    let hash = Sha256::new().
                    chain_update(input).
                    chain_update(timestamp_nanos.to_string()).
                    chain_update(rng.to_string())
                    .finalize();
                    
    encode(&hash)
}