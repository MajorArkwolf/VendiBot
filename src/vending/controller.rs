use crate::io::weigand::WeigandReader;

use super::selection::Selection;
use color_eyre::Result;
use super::super::IElectronicController;
use tokio::{sync::watch, task::JoinHandle};
use super::super::io::weigand;

pub struct Controller {
    drink_selection: Selection,
    rx: tokio::sync::watch::Receiver<weigand::Weigand>,
    is_running: bool,
}

impl Controller {
    pub async fn load_controller(io_controller: &mut Box<dyn IElectronicController>) -> Result<Self> {
        const GPIO_ZERO_LINE: u8 = 0;
        const GPIO_ONE_LINE: u8 = 1;
        let mut reader =
        WeigandReader::new(GPIO_ZERO_LINE, GPIO_ONE_LINE, io_controller)?;
        let (tx, rx) = watch::channel::<weigand::Weigand>(weigand::Weigand::new_unchecked(0));
        let _x: JoinHandle<Result<()>> = tokio::task::spawn(async move {
            reader.run(tx).await?;
            Ok(())
        });
        Ok(Self{drink_selection: Selection::load_data().await?, rx, is_running: true})
    }

    pub async fn run(&mut self) -> Result<()> {
        while self.is_running {
            if self.rx.has_changed()? {
                let message = self.rx.borrow_and_update();
                // wait for either key or bypass
                // validate key
                // dispense drink wait for selection or key
                // wait for knocksensor or timeout
                // notify of operation
            }

        }
        Ok(())
    }
}