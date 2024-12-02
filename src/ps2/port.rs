use defmt::*;
use embassy_rp::{
    gpio::Output,
    pio::{Instance as PioInstance, Pio, PioPin},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::TimeoutError;
use pc_keyboard::Ps2Decoder;
use rmk::keyboard::KeyEvent;

use super::{event_queue::EventQueue, pio::PioPs2Rx};

pub(crate) struct PS2IO<PIO: PioInstance + 'static> {
    pub(crate) port: PioPs2Rx<'static, PIO>,
    pub(crate) led: Output<'static>,
}

pub(crate) struct PS2Port<PIO: PioInstance + 'static> {
    pub(crate) ps2io: Mutex<CriticalSectionRawMutex, PS2IO<PIO>>,

    processor: Mutex<CriticalSectionRawMutex, (EventQueue<KeyEvent, 256>, ScancodeProcessor)>,

    ps2_decoder: Ps2Decoder,
}

#[derive(Format)]
enum ScancodeError {
    KeyOverrun,
    UnknownKey,
}
#[derive(PartialEq)]
enum DecodeState {
    Start,
    KeyRelease,
    // TODO: https://github.com/tmk/tmk_keyboard/wiki/IBM-PC-AT-Keyboard-Protocol#commands-to-the-system
}
struct ScancodeProcessor {
    state: DecodeState,
}
impl ScancodeProcessor {
    fn new() -> Self {
        Self {
            state: DecodeState::Start,
        }
    }

    fn advance_state(&mut self, code: u8) -> Result<Option<KeyEvent>, ScancodeError> {
        match code {
            0x00 => Err(ScancodeError::KeyOverrun),

            0xf0 => {
                self.state = DecodeState::KeyRelease;
                Ok(None)
            }

            _ => {
                let row: u8 = match code {
                    0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 | 0x40 | 0x48 | 0x50 | 0x57
                    | 0x5f => 0,
                    0x07 | 0x0f | 0x17 | 0x1f | 0x27 | 0x2f | 0x37 | 0x3f | 0x47 | 0x4f | 0x56
                    | 0x5e => 1,
                    0x05 | 0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x25 | 0x2e | 0x36 | 0x3d | 0x3e
                    | 0x46 | 0x45 | 0x4e | 0x55 | 0x66 | 0x67 | 0x6e | 0x6f | 0x76 | 0x77
                    | 0x7e | 0x84 => 2,
                    0x04 | 0x0c | 0x0d | 0x15 | 0x1d | 0x24 | 0x2d | 0x2c | 0x35 | 0x3c | 0x43
                    | 0x44 | 0x4d | 0x54 | 0x5b | 0x64 | 0x65 | 0x6d | 0x6c | 0x75 | 0x7d
                    | 0x7c => 3,
                    0x03 | 0x0b | 0x14 | 0x1c | 0x1b | 0x23 | 0x2b | 0x34 | 0x33 | 0x3b | 0x42
                    | 0x4b | 0x4c | 0x52 | 0x53 | 0x5a | 0x63 | 0x6b | 0x73 | 0x74 | 0x7b => 4,
                    0x83 | 0x0a | 0x12 | 0x13 | 0x1a | 0x22 | 0x21 | 0x2a | 0x32 | 0x31 | 0x3a
                    | 0x41 | 0x49 | 0x4a | 0x59 | 0x61 | 0x62 | 0x6a | 0x69 | 0x72 | 0x7a
                    | 0x79 => 5,
                    0x01 | 0x09 | 0x11 | 0x19 | 0x29 | 0x39 | 0x58 | 0x60 | 0x70 | 0x71 => 6,

                    _ => return Err(ScancodeError::UnknownKey),
                };

                let col: u8 = match code {
                    0x05 | 0x04 | 0x03 | 0x83 | 0x01 => 0,
                    0x06 | 0x0c | 0x0b | 0x0a | 0x09 => 1,

                    0x0e | 0x0d | 0x14 | 0x12 | 0x11 => 2,
                    0x08 | 0x07 | 0x16 | 0x15 | 0x1c | 0x13 => 3,
                    0x10 | 0x0f | 0x1e | 0x1d | 0x1b | 0x1a | 0x19 => 4,
                    0x18 | 0x17 | 0x26 | 0x24 | 0x23 | 0x22 => 5,
                    0x20 | 0x1f | 0x25 | 0x2d | 0x2b | 0x21 => 6,
                    0x28 | 0x27 | 0x2e | 0x2c | 0x34 | 0x2a => 7,
                    0x30 | 0x2f | 0x36 | 0x35 | 0x33 | 0x32 => 8,
                    0x38 | 0x37 | 0x3d | 0x3c | 0x3b | 0x31 | 0x29 => 9,
                    0x40 | 0x3f | 0x3e | 0x43 | 0x42 | 0x3a => 10,
                    0x48 | 0x47 | 0x46 | 0x44 | 0x4b | 0x41 => 11,
                    0x50 | 0x4f | 0x45 | 0x4d | 0x4c | 0x49 => 12,
                    0x57 | 0x56 | 0x4e | 0x54 | 0x52 | 0x4a | 0x39 => 13,
                    0x5f | 0x5e | 0x55 | 0x5b | 0x53 => 14,
                    0x66 | 0x5a | 0x59 | 0x58 => 15,

                    0x67 | 0x64 | 0x61 => 16,
                    0x6e | 0x65 | 0x63 | 0x62 | 0x60 => 17,
                    0x6f | 0x6d | 0x6a => 18,

                    0x76 | 0x6c | 0x6b | 0x69 => 19,
                    0x77 | 0x75 | 0x73 | 0x72 | 0x70 => 20,
                    0x7e | 0x7d | 0x74 | 0x7a | 0x71 => 21,
                    0x84 | 0x7c | 0x7b | 0x79 => 22,

                    _ => return Err(ScancodeError::UnknownKey),
                };

                let pressed = self.state != DecodeState::KeyRelease;
                self.state = DecodeState::Start;

                Ok(Some(KeyEvent { row, col, pressed }))
            }
        }
    }
}

impl<PIO: PioInstance> PS2Port<PIO> {
    pub fn new(
        pio: Pio<'static, PIO>,
        data_pin: impl PioPin,
        clk_pin: impl PioPin,
        led_pin: Output<'static>,
    ) -> Self {
        Self {
            ps2io: Mutex::new(PS2IO {
                port: PioPs2Rx::new(pio, data_pin, clk_pin),
                led: led_pin,
            }),

            processor: Mutex::new((
                EventQueue::new(KeyEvent {
                    row: 0,
                    col: 0,
                    pressed: false,
                }),
                ScancodeProcessor::new(),
            )),

            ps2_decoder: Ps2Decoder::new(),
            // scancode_processor: ScancodeSet2::new(),
        }
    }

    pub async fn pop_event(&self) -> Option<KeyEvent> {
        let mut processor = self.processor.lock().await;
        processor.0.pop()
    }

    /// wait for and decode the next PS/2 data packet
    /// if this completes a key event, add it to the event queue
    pub async fn decode_next(&self, pins: &mut PS2IO<PIO>) {
        if let Ok(ps2_data) = Self::get_ps2_data(pins).await {
            let decode_result = self.ps2_decoder.add_word(ps2_data);
            match decode_result {
                Ok(code) => {
                    info!("Scan code: {:x}", code);
                    let mut processor = self.processor.lock().await;
                    match processor.1.advance_state(code) {
                        Ok(Some(key_event)) => {
                            debug!("{}", key_event);
                            if processor.0.push(key_event).is_err() {
                                error!("Event queue is full!")
                            }
                        }
                        Ok(None) => {
                            // scan code without immediate effect (e.g. key release)
                        }
                        Err(e) => error!("Error processing PS/2 scan code: {:?}", e),
                    };
                }
                Err(e) => {
                    error!("Error decoding PS/2 data: {:?}", Debug2Format(&e));
                }
            }
        }
    }

    async fn get_ps2_data(ps2io: &mut PS2IO<PIO>) -> Result<u16, TimeoutError> {
        let data = ps2io.port.read_packet().await;
        // debug!("Got PS/2 packet: {:011b}", data);
        ps2io.led.toggle();
        Ok(data)
    }
}
