mod io;
mod vending;
mod artifactory;
use color_eyre::eyre::{Result};
use io::backend::IElectronicController;

#[cfg(all(target_arch = "aarch64", target_os = "linux", target_vendor="unknown", target_env="gnu"))]
fn generate() -> Result<Box<dyn IElectronicController>> {
    Ok(Box::new(io::backend::mock::pi_os::Controller::new()?))
}

#[cfg(not(all(target_arch = "aarch64", target_os = "linux", target_vendor="unknown", target_env="gnu")))]
fn generate() -> Result<Box<dyn IElectronicController>> {
    Ok(Box::new(io::backend::mock::Controller::new()?))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut io_controller = generate()?;
    let mut controller = vending::controller::Controller::load_controller(&mut io_controller).await?;
    controller.run().await?;
    Ok(())
}
