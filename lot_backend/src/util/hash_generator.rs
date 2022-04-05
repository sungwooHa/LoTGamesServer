
use rsa::{PublicKey, RsaPublicKey, RsaPrivateKey, PaddingScheme};

use chrono::prelude::*;
use chrono::{DateTime, Utc};
use rand::rngs::OsRng;

pub fn generate_hash_with_time(email : String) -> Result<String, &'static str> {
    let mut rng = OsRng;
    let bits = 500;

    let priv_key = RsaPrivateKey::new(&mut rng, bits)
                                .expect("Failed to generate hash : failed to generate a private key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // get timestamp (nano)
    let timestamp_nanos= Utc::now().timestamp_nanos();       // e.g. `2014-11-28T12:45:59.324310806Z`

    // Encrypt

    match pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(),
    &(email.to_string() + &timestamp_nanos.to_string()).as_bytes()){
        Ok(enc_data) => Ok(format!("{:?}", enc_data)),
        Err(err) => Err("Failed to generate hash : failed to encrypt"),
    }
}