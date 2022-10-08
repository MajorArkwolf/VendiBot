use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Drink {
    pub id: u8,
    pub name: String,
    pub cost: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrinkOption {
    pub drink_id: u8,
    pub input_pin: u8,
    pub output_pin: u8,
    pub is_valid: bool,
}