use defmt::{error, info, warn};
use embassy_rp::{
    gpio::Output,
    pio::{Instance as PioInstance, Pio, PioPin},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::TimeoutError;
use pc_keyboard::{KeyCode, KeyEvent, KeyState, Ps2Decoder, ScancodeSet, ScancodeSet2};

use super::pio::PioPs2Rx;

struct EventQueue<T: Copy, const S: usize> {
    write: usize,
    read: usize,

    events: [T; S],
}

impl<T: Copy, const S: usize> EventQueue<T, S> {
    fn new(init: T) -> Self {
        Self {
            events: [init; S],
            write: 0,
            read: 0,
        }
    }

    fn push(&mut self, event: T) -> Result<(), ()> {
        if (self.write + 1) % S == self.read {
            return Err(());
        }

        self.events[self.write] = event;
        self.write = (self.write + 1) % S;

        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.read < self.write {
            // TODO: handle overflow
            let event = self.events[self.read];
            self.read = (self.read + 1) % S;
            Some(event)
        } else {
            None
        }
    }
}

pub(crate) struct PS2IO<PIO: PioInstance + 'static> {
    port: PioPs2Rx<'static, PIO>,
    led: Output<'static>,
}

pub(crate) struct PS2Port<PIO: PioInstance + 'static> {
    pub(crate) pins: Mutex<CriticalSectionRawMutex, PS2IO<PIO>>,

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
            pins: Mutex::new(PS2IO {
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
                        Ok(None) => warn!("Scan code without effect??"),
                        Err(e) => error!("Error processing PS/2 scan code"),
                    };
                }
                Err(e) => {
                    error!("Error decoding PS/2 data");
                }
            }
        }
    }

    async fn get_ps2_data(ps2io: &mut PS2IO<PIO>) -> Result<u16, TimeoutError> {
        let data = ps2io.port.read_packet().await;
        info!("Got PS/2 packet: {:011b}", data);
        Ok(data)
    }
}
