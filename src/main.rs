#![no_main]
#![no_std]

#[macro_use]
mod keymap;
#[macro_use]
mod macros;
mod vial;

mod ps2;

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    flash::{Async, Flash},
    gpio::{Level, Output},
    peripherals::{PIO0, USB},
    pio::{self, Pio},
    usb::{self, Driver},
};
use keymap::{COL, ROW};
// use embassy_rp::flash::Blocking;
use panic_probe as _;
use ps2::{matrix::PS2Matrix, port::PS2Port};
use rmk::{
    config::{KeyboardUsbConfig, RmkConfig, VialConfig},
    run_rmk_custom_matrix,
};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});

const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[embassy_executor::task]
async fn ps2_background_read(port: &'static PS2Port<PIO0>) {
    info!("Begin PS/2 background task");
    let pins = &mut port.pins.lock().await;
    loop {
        port.decode_next(pins).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("RMK start!");
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Create the usb driver, from the HAL
    let driver = Driver::new(p.USB, Irqs);

    // Initialize PS/2 port
    let pio = Pio::new(p.PIO0, Irqs);
    let data_pin = p.PIN_2;
    let clk_pin = p.PIN_3;
    let led_pin = Output::new(p.PIN_25, Level::Low);
    let ps2_port = PS2Port::new(pio, data_pin, clk_pin, led_pin);
    static PS2_PORT: StaticCell<PS2Port<PIO0>> = StaticCell::new();
    let ps2_port = PS2_PORT.init(ps2_port);

    // Create key matrix
    let matrix: PS2Matrix<ROW, COL, PIO0> = PS2Matrix::new(ps2_port);

    // Use internal flash to emulate eeprom
    // Both blocking and async flash are support, use different API
    // let flash = Flash::<_, Blocking, FLASH_SIZE>::new_blocking(p.FLASH);
    let flash = Flash::<_, Async, FLASH_SIZE>::new(p.FLASH, p.DMA_CH0);

    let keyboard_usb_config = KeyboardUsbConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "Haobo",
        product_name: "RMK Keyboard",
        serial_number: "vial:f64c2b3c:000001",
    };

    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF);

    let keyboard_config: RmkConfig<Output<'static>> = RmkConfig {
        usb_config: keyboard_usb_config,
        vial_config,
        ..Default::default()
    };

    // Set up background process to read bit-banged PS/2 data
    spawner.must_spawn(ps2_background_read(ps2_port));

    // Start serving
    run_rmk_custom_matrix(
        matrix,
        driver,
        flash,
        &mut keymap::get_default_keymap(),
        keyboard_config,
        spawner,
    )
    .await;
}
