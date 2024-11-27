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
    gpio::{Input, Level, Output, Pull},
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};
use embassy_sync::mutex::Mutex;
use keymap::{COL, ROW};
// use embassy_rp::flash::Blocking;
use panic_probe as _;
use ps2::{
    matrix::{PS2AsyncMutex, PS2Matrix},
    port::PS2Port,
};
use rmk::{
    config::{KeyboardUsbConfig, RmkConfig, VialConfig},
    run_rmk_custom_matrix,
};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[embassy_executor::task]
async fn ps2_background_read(port: &'static PS2AsyncMutex) {
    loop {
        port.lock().await.decode_next().await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("RMK start!");
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Create the usb driver, from the HAL
    let driver = Driver::new(p.USB, Irqs);

    let led_pin = Output::new(p.PIN_25, Level::Low);

    // Initialize PS/2 port
    let clk_pin = Input::new(p.PIN_2, Pull::Up);
    let data_pin = Input::new(p.PIN_3, Pull::Up);
    let ps2_port = PS2Port::new(clk_pin, data_pin, led_pin);
    static PS2_PORT: StaticCell<PS2AsyncMutex> = StaticCell::new();
    let ps2_port = PS2_PORT.init(Mutex::new(ps2_port));

    // Set up background process to read bit-banged PS/2 data
    spawner.must_spawn(ps2_background_read(ps2_port));

    // Create key matrix
    let matrix: PS2Matrix<ROW, COL> = PS2Matrix::new(ps2_port);

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
