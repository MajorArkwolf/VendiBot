use super::selection::Selection;
use color_eyre::Result;

pub struct Controller {
    drink_selection: Selection,
    is_running: bool,
}

impl Controller {
    pub async fn load_controller() -> Result<Self> {
        Ok(Self{drink_selection: Selection::load_data().await?, is_running: true})
    }

    pub async fn run(&mut self) -> Result<()> {
        while self.is_running {
            // wait for either key or bypass
            // dispense drink wait for selection or key
            // wait for knocksensor or timeout
            // notify of operation
            
        }
        Ok(())
    }
}