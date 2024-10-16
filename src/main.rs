#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can::{
    Can, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler,
};
use stm32_telemetry::messages::Messages;
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::peripherals::CAN;

bind_interrupts!(struct Irqs {
    USB_LP_CAN_RX0 => Rx0InterruptHandler<CAN>;
    CAN_RX1 => Rx1InterruptHandler<CAN>;
    CAN_SCE => SceInterruptHandler<CAN>;
    USB_HP_CAN_TX => TxInterruptHandler<CAN>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // Configure CAN inteface
    let mut driver = Can::new(p.CAN, p.PB8, p.PB9, Irqs);

    // Configure the CAN bitrate to 1Mbit
    driver.set_bitrate(1_000_000);

    loop {
        // TODO: what is the proper way to bubble an error to main?
        let msg = driver.read().await.unwrap();

        // Note, this is expensive and just done for ease here
        info!("Received message with id {}", Debug2Format(&msg.frame.id()));

        // TODO: this seems rather gross, I think all messages will be extended ids
        let raw_id = match msg.frame.id() {
            embassy_stm32::can::bxcan::Id::Standard(standard_id) => standard_id.as_raw() as u32,
            embassy_stm32::can::bxcan::Id::Extended(extended_id) => extended_id.as_raw(),
        };

        if let Ok(stm32_telemetry::messages::Messages::Dme1(dme1)) =
            // Note that data derefs to [u8]
            Messages::from_can_message(raw_id, msg.frame.data().unwrap())
        {
            info!("Received DME1: {:?}", Debug2Format(&dme1));
        } else {
            info!("Failed to parse frame");
        }
    }
}
