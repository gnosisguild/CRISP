
use std::{str, sync::Arc, error::Error};
use once_cell::sync::Lazy;
use sled::{Db, IVec};
use rand::Rng;
use log::info;
use super::models::{Round, E3};

pub static GLOBAL_DB: Lazy<Arc<Db>> = Lazy::new(|| {
    let pathdb = std::env::current_dir().unwrap().join("database/enclave_server");
    Arc::new(sled::open(pathdb).unwrap())
});
pub fn get_e3(e3_id: u64) -> Result<(E3, String), Box<dyn Error>> {
    let key = format!("e3:{}", e3_id);

    let value = match GLOBAL_DB.get(key.clone()) {
        Ok(Some(v)) => v,                 
        Ok(None) => return Err(format!("E3 not found: {}", key).into()), 
        Err(e) => return Err(format!("Database error: {}", e).into()),
    };

    let e3: E3 = serde_json::from_slice(&value)
        .map_err(|e| format!("Failed to deserialize E3: {}", e))?;

    Ok((e3, key))
}

pub fn get_e3_round() -> Result<u64, Box<dyn Error>> {
    let key = "e3:round";

    let round_count: u64 = match GLOBAL_DB.get(key) {
        Ok(Some(bytes)) => {
            match bincode::deserialize::<u64>(&bytes) {
                Ok(count) => count,
                Err(e) => {
                    info!("Failed to deserialize round count: {}", e);
                    return Err(format!("Failed to retrieve round count").into());
                }
            }
        }
        Ok(None) => {
            info!("Initializing first round in db");
            let initial_count = 0u64;
            let encoded = bincode::serialize(&initial_count).unwrap();
            if let Err(e) = GLOBAL_DB.insert(key, IVec::from(encoded)) {
                info!("Failed to initialize first round in db: {}", e);
                return Err(format!("Failed to initialize round count").into());
            }
            initial_count
        }
        Err(e) => {
            info!("Database error: {}", e);
            return Err(format!("Database error").into());
        }
    };

    Ok(round_count)
}


pub fn increment_e3_round() -> Result<(), Box<dyn Error>> {
    let key = "e3:round";

    match get_e3_round() {
        Ok(round_count) => {
            let new_round_count = round_count + 1;
            let encoded = bincode::serialize(&new_round_count).unwrap();
            GLOBAL_DB.insert(key, IVec::from(encoded))?;
        }
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}

pub fn get_state(round_id: u32) -> (Round, String) {
    let mut round_key = round_id.to_string();
    round_key.push_str("-storage");
    info!("Database key is {:?}", round_key);
    let state_out = GLOBAL_DB.get(round_key.clone()).unwrap().unwrap();
    let state_out_str = str::from_utf8(&state_out).unwrap();
    let state_out_struct: Round = serde_json::from_str(&state_out_str).unwrap();
    (state_out_struct, round_key)
}

pub fn get_round_count() -> u32 {
    let round_key = "round_count";
    let round_db = GLOBAL_DB.get(round_key).unwrap();
    if round_db == None {
        info!("initializing first round in db");
        GLOBAL_DB.insert(round_key, b"0".to_vec()).unwrap();
    }
    let round_str = std::str::from_utf8(round_db.unwrap().as_ref()).unwrap().to_string();
    round_str.parse::<u32>().unwrap()
}

pub fn generate_emoji() -> (String, String) {
    let emojis = [
        "🍇","🍈","🍉","🍊","🍋","🍌","🍍","🥭","🍎","🍏",
        "🍐","🍑","🍒","🍓","🫐","🥝","🍅","🫒","🥥","🥑",
        "🍆","🥔","🥕","🌽","🌶️","🫑","🥒","🥬","🥦","🧄",
        "🧅","🍄","🥜","🫘","🌰","🍞","🥐","🥖","🫓","🥨",
        "🥯","🥞","🧇","🧀","🍖","🍗","🥩","🥓","🍔","🍟",
        "🍕","🌭","🥪","🌮","🌯","🫔","🥙","🧆","🥚","🍳",
        "🥘","🍲","🫕","🥣","🥗","🍿","🧈","🧂","🥫","🍱",
        "🍘","🍙","🍚","🍛","🍜","🍝","🍠","🍢","🍣","🍤",
        "🍥","🥮","🍡","🥟","🥠","🥡","🦀","🦞","🦐","🦑",
        "🦪","🍦","🍧","🍨","🍩","🍪","🎂","🍰","🧁","🥧",
        "🍫","🍬","🍭","🍮","🍯","🍼","🥛","☕","🍵","🍾",
        "🍷","🍸","🍹","🍺","🍻","🥂","🥃",
    ];
    let mut index1 = rand::thread_rng().gen_range(0..emojis.len());
    let index2 = rand::thread_rng().gen_range(0..emojis.len());
    if index1 == index2 {
        if index1 == emojis.len() {
            index1 = index1 - 1;
        } else {
            index1 = index1 + 1;
        };
    };
    (emojis[index1].to_string(), emojis[index2].to_string())
}

pub fn pick_response() -> String {
    "Test".to_string()
}