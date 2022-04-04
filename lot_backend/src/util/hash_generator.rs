
use rsa::{PublicKey, RsaPublicKey, RsaPrivateKey, PaddingScheme};

use chrono::prelude::*;
use chrono::{DateTime, Utc};
use rand::rngs::OsRng;

pub fn generate_hash(email : String) -> Option<String>{
    let mut rng = OsRng;
    let bits = 2048;

    let priv_key = RsaPrivateKey::new(&mut rng, bits).
                    expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);


    // get current time.
    let utc : DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
    let dt = utc.timestamp();

    println!("current utc : {}", utc);
    println!("current time : {}", dt);

    // Encrypt
    let data = email.to_string() + &utc.to_string();
    println!("src data : {}", data);
    match std::str::from_utf8(&pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(),
                     &data.as_bytes()[..]).expect("failed to encrypt")){
                         Ok (hash_data) => Some(hash_data.to_string()),
                         Err(e) =>  panic!("Invalid UTF-8 sequence: {}", e),
                     }
}