mod io;
mod vending;
use color_eyre::eyre::{Result};

#[tokio::main]
async fn main() -> Result<()> {
    let controller = vending::controller::Controller::load_controller().await?;
    println!("Hello, world!");
    Ok(())
}
