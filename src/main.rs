#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::can::filter::Mask32;
use embassy_stm32::can::{
    Can, Fifo, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler,
};
use embassy_stm32::{bind_interrupts, peripherals};
use stm32_telemetry::messages::Messages;
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::peripherals::CAN;
use embassy_stm32::usart::{self, Config, Uart};

// Setup CAN interrupts
bind_interrupts!(struct Irqs {
    USB_LP_CAN_RX0 => Rx0InterruptHandler<CAN>;
    CAN_RX1 => Rx1InterruptHandler<CAN>;
    CAN_SCE => SceInterruptHandler<CAN>;
    USB_HP_CAN_TX => TxInterruptHandler<CAN>;
});

// Setup USART interrupts
bind_interrupts!(struct UartIrqs {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});

use telemetry_types::{postcard, Telemetry};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // Create UART
    let config = Config::default();

    let mut usart = Uart::new(
        p.USART2, p.PA3, p.PA2, UartIrqs, p.DMA1_CH7, p.DMA1_CH6, config,
    )
    .unwrap();

    // Configure CAN interface
    // PB8 is RX
    // PB9 is Tx
    let mut can = Can::new(p.CAN, p.PB8, p.PB9, Irqs);

    // Accept all messages
    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());

    // Configure the CAN bitrate to 500K
    can.set_bitrate(500_000);

    can.enable().await;

    info!("Initialized CAN driver");

    loop {
        // TODO: what is the proper way to bubble an error to main?
        let msg = can.read().await;

        if let Ok(msg) = msg {
            // TODO: this seems rather gross, I think all messages will be extended ids
            let raw_id = match msg.frame.id() {
                embassy_stm32::can::Id::Standard(standard_id) => standard_id.as_raw() as u32,
                embassy_stm32::can::Id::Extended(extended_id) => extended_id.as_raw(),
            };

            if let Ok(stm32_telemetry::messages::Messages::Dme1(dme1)) =
                // Note that data derefs to [u8]
                Messages::from_can_message(raw_id, msg.frame.data())
            {
                info!("Engine rpm: {}", dme1.rpm());

                // Write engine rpm to usart
                let telem = Telemetry {
                    rpm: dme1.rpm() as u32,
                };
                let mut buf = [0; 100];
                let bytes = postcard::to_slice(&telem, buf.as_mut_slice()).unwrap();
                usart.write(bytes).await.unwrap();
            } else {
                info!("Failed to parse frame");
            }
        }
    }
}
