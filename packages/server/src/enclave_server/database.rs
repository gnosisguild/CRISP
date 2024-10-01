use super::models::E3;
use log::info;
use once_cell::sync::Lazy;
use rand::Rng;
use sled::{Db, IVec};
use std::{error::Error, str, sync::Arc};
use tokio::sync::RwLock;

pub static GLOBAL_DB: Lazy<Arc<RwLock<Db>>> = Lazy::new(|| {
    let pathdb = std::env::current_dir()
        .unwrap()
        .join("database/enclave_server");
    Arc::new(RwLock::new(sled::open(pathdb).unwrap()))
});

pub async fn get_e3(e3_id: u64) -> Result<(E3, String), Box<dyn Error + Send + Sync>> {
    let key = format!("e3:{}", e3_id);

    let db = GLOBAL_DB.read().await;

    let value = match db.get(key.clone()) {
        Ok(Some(v)) => v,
        Ok(None) => return Err(format!("E3 not found: {}", key).into()),
        Err(e) => return Err(format!("Database error: {}", e).into()),
    };

    let e3: E3 =
        serde_json::from_slice(&value).map_err(|e| format!("Failed to deserialize E3: {}", e))?;

    Ok((e3, key))
}

pub async fn save_e3(e3: &E3, key: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let db = GLOBAL_DB.write().await;
    match db.insert(key.to_string(), serde_json::to_vec(e3)?) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to save E3: {}", e).into()),
    };
    Ok(())
}

pub async fn get_e3_round() -> Result<u64, Box<dyn Error + Send + Sync>> {
    let key = "e3:round";

    let db = GLOBAL_DB.read().await;

    let round_count: u64 = match db.get(key) {
        Ok(Some(bytes)) => match bincode::deserialize::<u64>(&bytes) {
            Ok(count) => count,
            Err(e) => {
                info!("Failed to deserialize round count: {}", e);
                return Err(format!("Failed to retrieve round count").into());
            }
        },
        Ok(None) => {
            drop(db);
            let db = GLOBAL_DB.write().await;

            info!("Initializing first round in db");
            let initial_count = 0u64;
            let encoded = bincode::serialize(&initial_count).unwrap();
            if let Err(e) = db.insert(key, IVec::from(encoded)) {
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

pub async fn increment_e3_round() -> Result<(), Box<dyn Error>> {
    let key = "e3:round";

    let new_round_count = match get_e3_round().await {
        Ok(round_count) => round_count + 1,
        Err(e) => return Err(e),
    };

    let db = GLOBAL_DB.write().await;
    db.insert(key, IVec::from(bincode::serialize(&new_round_count)?))?;

    Ok(())
}

pub fn generate_emoji() -> (String, String) {
    let emojis = [
        "🍇", "🍈", "🍉", "🍊", "🍋", "🍌", "🍍", "🥭", "🍎", "🍏", "🍐", "🍑", "🍒", "🍓", "🫐",
        "🥝", "🍅", "🫒", "🥥", "🥑", "🍆", "🥔", "🥕", "🌽", "🌶️", "🫑", "🥒", "🥬", "🥦", "🧄",
        "🧅", "🍄", "🥜", "🫘", "🌰", "🍞", "🥐", "🥖", "🫓", "🥨", "🥯", "🥞", "🧇", "🧀", "🍖",
        "🍗", "🥩", "🥓", "🍔", "🍟", "🍕", "🌭", "🥪", "🌮", "🌯", "🫔", "🥙", "🧆", "🥚", "🍳",
        "🥘", "🍲", "🫕", "🥣", "🥗", "🍿", "🧈", "🧂", "🥫", "🍱", "🍘", "🍙", "🍚", "🍛", "🍜",
        "🍝", "🍠", "🍢", "🍣", "🍤", "🍥", "🥮", "🍡", "🥟", "🥠", "🥡", "🦀", "🦞", "🦐", "🦑",
        "🦪", "🍦", "🍧", "🍨", "🍩", "🍪", "🎂", "🍰", "🧁", "🥧", "🍫", "🍬", "🍭", "🍮", "🍯",
        "🍼", "🥛", "☕", "🍵", "🍾", "🍷", "🍸", "🍹", "🍺", "🍻", "🥂", "🥃",
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
