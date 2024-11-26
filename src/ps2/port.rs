use defmt::{error, info, warn};
use embassy_rp::gpio::{Input, Output};
use embassy_time::{with_timeout, Duration, TimeoutError};
use pc_keyboard::{KeyCode, KeyEvent, KeyState, Ps2Decoder, ScancodeSet, ScancodeSet2};

pub(crate) struct PS2Port {
    clk_pin: Input<'static>,
    data_pin: Input<'static>,

    led_pin: Output<'static>,

    event_queue: [(KeyCode, KeyState); 256],
    event_queue_write: usize,
    event_queue_read: usize,

    ps2_decoder: Ps2Decoder,
    scancode_processor: ScancodeSet2,
}

impl PS2Port {
    pub fn new(clk_pin: Input<'static>, data_pin: Input<'static>, led_pin: Output<'static>) -> Self {
        Self {
            clk_pin,
            data_pin,

            led_pin,

            event_queue: [(pc_keyboard::KeyCode::A, pc_keyboard::KeyState::Up); 256],
            event_queue_write: 0,
            event_queue_read: 0,

            ps2_decoder: Ps2Decoder::new(),
            scancode_processor: ScancodeSet2::new(),
        }
    }

    pub fn pop_event(&mut self) -> Option<KeyEvent> {
        if self.event_queue_read < self.event_queue_write {
            // TODO: handle overflow
            let (code, state) = self.event_queue[self.event_queue_read];
            self.event_queue_read += 1;
            Some(KeyEvent { code, state })
        } else {
            None
        }
    }

    /// wait for and decode the next PS/2 data packet
    /// if this completes a key event, add it to the event queue
    pub async fn decode_next(&mut self) {
        if let Ok(ps2_data) = self.get_ps2_data().await {
            let decode_result = self.ps2_decoder.add_word(ps2_data);
            match decode_result {
                Ok(code) => {
                    match self.scancode_processor.advance_state(code) {
                        Ok(Some(KeyEvent { code, state })) => {
                            self.event_queue[self.event_queue_write] = (code, state);
                            self.event_queue_write += 1;
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

    async fn get_ps2_data(&mut self) -> Result<u16, TimeoutError> {
        self.clk_pin.wait_for_falling_edge().await;
        info!("Got start of PS/2 packet");

        self.led_pin.set_high();

        let mut data: u16 = 0;
        // read 1 start bit, 8 data bits, 1 parity bit, 1 stop bit
        for _ in 0..11 {
            match with_timeout(
                Duration::from_millis(1000),
                self.clk_pin.wait_for_rising_edge(),
            )
            .await
            {
                Ok(_) => data = (data << 1) | (self.data_pin.is_high() as u16),

                Err(e) => {
                    error!("Timeout while reading PS/2 packet");
                    self.led_pin.set_low();
                    return Err(e);
                }
            }
        }

        self.led_pin.set_low();

        Ok(data)
    }
}
