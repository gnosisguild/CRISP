
use std::{env, str, sync::Arc};
use once_cell::sync::Lazy;
use sled::Db;
use rand::Rng;
use log::info;
use super::models::Round;

pub static GLOBAL_DB: Lazy<Arc<Db>> = Lazy::new(|| {
    let pathdb = std::env::current_dir().unwrap().join("database/enclave_server");
    Arc::new(sled::open(pathdb).unwrap())
});

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