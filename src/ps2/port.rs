use core::marker::Copy;
use defmt::{error, info, warn};
use embassy_rp::gpio::{Input, Output};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::{with_timeout, Duration, Instant, TimeoutError};
use pc_keyboard::{KeyCode, KeyEvent, KeyState, Ps2Decoder, ScancodeSet, ScancodeSet2};

struct EventQueue<T: Copy, const S: usize> {
    write: usize,
    read: usize,

    events: [T; S],
}

impl<T: Copy, const S: usize> EventQueue<T, S> {
    pub fn new(init: T) -> Self {
        Self {
            events: [init; S],
            write: 0,
            read: 0,
        }
    }

    pub fn push(&mut self, event: T) {
        self.events[self.write] = event;
        self.write += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.read < self.write {
            // TODO: handle overflow
            let event = self.events[self.read];
            self.read += 1;
            Some(event)
        } else {
            None
        }
    }
}

pub(crate) struct Pins {
    clk: Input<'static>,
    data: Input<'static>,

    led: Output<'static>,
}

pub(crate) struct PS2Port {
    pub(crate) pins: Mutex<CriticalSectionRawMutex, Pins>,

    processor: Mutex<CriticalSectionRawMutex, (EventQueue<(KeyCode, KeyState), 256>, ScancodeSet2)>,

    ps2_decoder: Ps2Decoder,
    // scancode_processor: ScancodeSet2,
}

impl PS2Port {
    pub fn new(
        clk_pin: Input<'static>,
        data_pin: Input<'static>,
        led_pin: Output<'static>,
    ) -> Self {
        Self {
            pins: Mutex::new(Pins {
                clk: clk_pin,
                data: data_pin,
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
    pub async fn decode_next(&self, pins: &mut Pins) {
        if let Ok(ps2_data) = Self::get_ps2_data(pins) {
            let decode_result = self.ps2_decoder.add_word(ps2_data);
            match decode_result {
                Ok(code) => {
                    let mut processor = self.processor.lock().await;
                    match processor.1.advance_state(code) {
                        Ok(Some(KeyEvent { code, state })) => processor.0.push((code, state)),
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

    fn get_ps2_data(pins: &mut Pins) -> Result<u16, TimeoutError> {
        // TODO: timeout in millis
        let timeout = Duration::from_millis(1000);

        // wait for falling clock edge (start of PS/2 transmission)
        let start = Instant::now();
        while pins.clk.is_high() {
            let now = Instant::now();
            if now - start >= timeout {
                // pins.led.set_low();
                return Err(TimeoutError);
            }
        }
        info!("Got start of PS/2 packet");

        pins.led.set_high();

        let mut data: u16 = 0;
        // read 1 start bit, 8 data bits, 1 parity bit, 1 stop bit
        for _ in 0..11 {
            let start = Instant::now();

            // read on rising clock edge
            while pins.clk.is_high() {
                let now = Instant::now();
                if now - start >= timeout {
                    // pins.led.set_low();
                    return Err(TimeoutError);
                }
            }
            while pins.clk.is_low() {
                let now = Instant::now();
                if now - start >= timeout {
                    // pins.led.set_low();
                    return Err(TimeoutError);
                }
            }
            data = (data << 1) | (pins.data.is_high() as u16);
        }

        pins.led.set_low();

        Ok(data)
    }
}
