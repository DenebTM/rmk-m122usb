use defmt::{error, info, warn};
use embassy_rp::gpio::{Input, Output};
use embassy_time::{with_timeout, Duration, TimeoutError};
use pc_keyboard::{KeyCode, KeyEvent, KeyState, Ps2Decoder, ScancodeSet, ScancodeSet2};

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

    fn push(&mut self, event: T) {
        self.events[self.write] = event;
        self.write = (self.write + 1) % S;
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

pub(crate) struct PS2Port {
    clk_pin: Input<'static>,
    data_pin: Input<'static>,

    led_pin: Output<'static>,

    event_queue: EventQueue<(KeyCode, KeyState), 256>,

    ps2_decoder: Ps2Decoder,
    scancode_processor: ScancodeSet2,
}

impl PS2Port {
    pub fn new(
        clk_pin: Input<'static>,
        data_pin: Input<'static>,
        led_pin: Output<'static>,
    ) -> Self {
        Self {
            clk_pin,
            data_pin,
            led_pin,
            event_queue: EventQueue::new((pc_keyboard::KeyCode::A, pc_keyboard::KeyState::Up)),
            scancode_processor: ScancodeSet2::new(),
            ps2_decoder: Ps2Decoder::new(),
        }
    }

    pub fn pop_event(&mut self) -> Option<KeyEvent> {
        self.event_queue
            .pop()
            .map(|(code, state)| KeyEvent { code, state })
    }

    /// wait for and decode the next PS/2 data packet
    /// if this completes a key event, add it to the event queue
    pub async fn decode_next(&mut self) {
        if let Ok(ps2_data) = self.get_ps2_data().await {
            let decode_result = self.ps2_decoder.add_word(ps2_data);
            match decode_result {
                Ok(code) => {
                    match self.scancode_processor.advance_state(code) {
                        Ok(Some(KeyEvent { code, state })) => self.event_queue.push((code, state)),
                        Ok(None) => warn!("Scan code without effect??"),
                        Err(e) => error!(
                            "Error processing PS/2 scan code: {:?}",
                            defmt::Debug2Format(&e)
                        ),
                    };
                }
                Err(e) => {
                    error!("Error decoding PS/2 data: {:?}", defmt::Debug2Format(&e));
                }
            }
        }
    }

    async fn get_ps2_data(&mut self) -> Result<u16, TimeoutError> {
        self.clk_pin.wait_for_falling_edge().await;

        self.led_pin.set_high();

        let mut bits_got = 0;

        let mut data: u16 = 0;
        // read 1 start bit, 8 data bits, 1 parity bit, 1 stop bit
        for _ in 0..11 {
            match with_timeout(
                Duration::from_millis(10),
                self.clk_pin.wait_for_rising_edge(),
            )
            .await
            {
                Ok(_) => {
                    data = ((self.data_pin.is_high() as u16) << 10) | (data >> 1);
                    bits_got += 1;
                }

                r @ Err(_) => {
                    error!("Incomplete PS/2 packet; got {} bits", bits_got);
                    self.led_pin.set_low();
                    r?
                }
            }
        }

        self.led_pin.set_low();

        info!("Got PS/2 packet: {:011b}", data);
        Ok(data)
    }
}
