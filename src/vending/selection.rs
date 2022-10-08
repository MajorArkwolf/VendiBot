use super::drink_option::{DrinkOption, Drink};
use color_eyre::eyre::{Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::{fs::{self, File}, io::AsyncReadExt};

#[derive(Debug, Serialize, Deserialize)]
struct DrinkArray {
    pub drinks: Vec<Drink>,
    pub drink_options: Vec<DrinkOption>,
}

pub struct Selection {
    drink_info: HashMap<u8, Drink>,
    selection_map: HashMap<u8, DrinkOption>,
}

impl Selection {
    pub async fn load_data() -> Result<Self> {
        let mut buffer = Vec::new();
        File::open("./data/drinks.json").await?.read_to_end(&mut buffer).await?;
        let config: DrinkArray = serde_json::from_str(std::str::from_utf8(&buffer)?)?;
        let drink_info = config.drinks.into_iter().map(|f| (f.id, f)).collect::<HashMap<u8, Drink>>();
        let selection_map = config.drink_options.into_iter().map(|f| (f.drink_id, f)).collect::<HashMap<u8, DrinkOption>>();
        Ok(Self{drink_info, selection_map})
    }
}