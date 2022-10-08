use super::backend::IElectronicController;
use super::backend::Level;
use super::backend::PinPull;
use super::backend::Trigger;
use super::timer::Timer;
use bit_field::BitField;
use color_eyre::eyre::{eyre, Result};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error};

enum Transmission {
    None,
    Payload(u32),
}

pub struct WeigandReader {
    rx: Receiver<u8>,
}

impl WeigandReader {
    pub fn new(
        zero_pin: u8,
        one_pin: u8,
        controller: &mut Box<dyn IElectronicController>,
    ) -> Result<Self> {
        let zero_pin = controller.setup_input_pin(zero_pin, PinPull::PullUp)?;
        let one_pin = controller.setup_input_pin(one_pin, PinPull::PullUp)?;
        let (one_tx, rx) = mpsc::channel::<u8>(200);

        let zero_tx = one_tx.clone();
        let zero_call = move |level: Level| {
            match level {
                Level::Low => match zero_tx.try_send(0) {
                    Ok(_) => {}
                    Err(e) => error!("failed to send low bit: {}", e),
                },
                Level::High => {}
            };
        };

        controller.set_async_interrupt(zero_pin, Trigger::FallingEdge, Box::new(zero_call))?;

        let one_call = move |level: Level| {
            match level {
                Level::Low => match one_tx.try_send(1) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("failed to send high bit: {}", e)
                    }
                },
                Level::High => {}
            };
        };
        controller.set_async_interrupt(one_pin, Trigger::FallingEdge, Box::new(one_call))?;

        Ok(Self { rx })
    }

    pub async fn run(&mut self, channel: tokio::sync::watch::Sender<Weigand>) -> Result<()> {
        debug!("weigand reader beginning to run");
        loop {
            match self.get_payload().await? {
                Transmission::None => { /* Most likely a timeout */ }
                Transmission::Payload(data) => match Weigand::new(data) {
                    Ok(weigand) => match channel.send(weigand) {
                        Ok(_) => {}
                        Err(e) => error!("failed to send wiegand payload: {}", e),
                    },
                    Err(e) => debug!("error getting payload: {}", e),
                },
            }
        }
    }

    async fn get_payload(&mut self) -> Result<Transmission> {
        let max_duration = std::time::Duration::new(0, 500);
        let mut buffer: u32 = 0;
        let mut bit_counter = 0;
        let mut timer = Timer::default();

        while bit_counter < 32 {
            match self.rx.recv().await {
                Some(byte) => {
                    buffer <<= 1;
                    if byte > 0 {
                        buffer |= 1;
                    }
                    bit_counter += 1;
                    timer.reset();
                    debug!("Recv Byte: {}", byte);
                }
                None => {
                    return Err(eyre!("channel closed unexpectadly"));
                }
            }
            if timer.progress(std::time::Instant::now()) > max_duration {
                debug!("timer elapsed on transmission, terminating connection");
                return Ok(Transmission::None);
            }
        }

        debug!(
            "transmission recv ({}), forwarding payload to be processed",
            buffer
        );
        Ok(Transmission::Payload(buffer))
    }
}

#[derive(Debug)]
pub struct Weigand {
    facility_code: u16,
    card_number: u32,
}

impl Weigand {
    pub fn new(data: u32) -> Result<Self> {
        let parity_even = bit_field::BitField::get_bit(&data, 0) as bool;
        let parity_odd = bit_field::BitField::get_bit(&data, 31) as bool;

        let even_calc_bit = (Weigand::count_ones(data, 1, 17) % 2) == 0;
        let odd_calc_bit = (Weigand::count_ones(data, 18, 30) % 2) == 1;

        if parity_even != even_calc_bit {
            return Err(eyre!(
                "odd parity bit was incorrect, Expected: {}, Calculated: {}",
                parity_even,
                even_calc_bit,
            ));
        }

        if parity_odd != odd_calc_bit {
            return Err(eyre!(
                "odd parity bit was incorrect, Expected: {}, Calculated: {}",
                parity_odd,
                odd_calc_bit,
            ));
        }
        Ok(Weigand::new_unchecked(data))
    }

    pub fn new_unchecked(data: u32) -> Self {
        let facility_code = data.get_bits(1..8) as u16;
        let card_number = data.get_bits(9..30) as u32;

        Self {
            facility_code,
            card_number,
        }
    }

    fn count_ones(data: u32, start_index: usize, end_index: usize) -> usize {
        let mut counter: usize = 0;
        for i in start_index..end_index {
            if data.get_bit(i) {
                counter += 1;
            }
        }
        counter
    }

    pub fn get_facility_code(&self) -> u16 {
        self.facility_code
    }

    pub fn get_card_number(&self) -> u32 {
        self.card_number
    }
}