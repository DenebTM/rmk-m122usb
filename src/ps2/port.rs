use defmt::*;
use embassy_rp::{
    gpio::Output,
    pio::{Instance as PioInstance, Pio, PioPin},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::TimeoutError;
use pc_keyboard::{KeyCode, KeyEvent, KeyState, Ps2Decoder, ScancodeSet, ScancodeSet2};

use super::{event_queue::EventQueue, pio::PioPs2Rx};


pub(crate) struct PS2IO<PIO: PioInstance + 'static> {
    pub(crate) port: PioPs2Rx<'static, PIO>,
    pub(crate) led: Output<'static>,
}

pub(crate) struct PS2Port<PIO: PioInstance + 'static> {
    pub(crate) ps2io: Mutex<CriticalSectionRawMutex, PS2IO<PIO>>,

    processor: Mutex<CriticalSectionRawMutex, (EventQueue<(KeyCode, KeyState), 256>, ScancodeSet2)>,

    ps2_decoder: Ps2Decoder,
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
                EventQueue::new((pc_keyboard::KeyCode::A, pc_keyboard::KeyState::Up)),
                ScancodeSet2::new(),
            )),

            ps2_decoder: Ps2Decoder::new(),
            // scancode_processor: ScancodeSet2::new(),
        }
    }

    pub async fn pop_event(&self) -> Option<KeyEvent> {
        let mut processor = self.processor.lock().await;
        processor
            .0
            .pop()
            .map(|(code, state)| KeyEvent { code, state })
    }

    /// wait for and decode the next PS/2 data packet
    /// if this completes a key event, add it to the event queue
    pub async fn decode_next(&self, pins: &mut PS2IO<PIO>) {
        if let Ok(ps2_data) = Self::get_ps2_data(pins).await {
            let decode_result = self.ps2_decoder.add_word(ps2_data);
            match decode_result {
                Ok(code) => {
                    let mut processor = self.processor.lock().await;
                    match processor.1.advance_state(code) {
                        Ok(Some(KeyEvent { code, state })) => {
                            if processor.0.push((code, state)).is_err() {
                                error!("Event queue is full!")
                            }
                        }
                        Ok(None) => {
                            // scan code without immediate effect (e.g. key release)
                        },
                        Err(e) => error!("Error processing PS/2 scan code: {:?}", Debug2Format(&e)),
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
