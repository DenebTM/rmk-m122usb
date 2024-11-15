use defmt::{error, warn};
use pc_keyboard::{KeyEvent, KeyCode, KeyState, Ps2Decoder, ScancodeSet, ScancodeSet2};

pub(crate) struct PS2Port {
    clk_pin: u16,
    data_pin: u16,

    event_queue: [(KeyCode, KeyState); 256],
    event_queue_write: usize,
    event_queue_read: usize,

    ps2_decoder: Ps2Decoder,
    scancode_processor: ScancodeSet2,
}

impl PS2Port {
    pub fn new(clk_pin: u16, data_pin: u16) -> Self {
        Self {
            clk_pin,
            data_pin,

            event_queue: [(pc_keyboard::KeyCode::A, pc_keyboard::KeyState::Up); 256],
            event_queue_write: 0,
            event_queue_read: 0,

            ps2_decoder: Ps2Decoder::new(),
            scancode_processor: ScancodeSet2::new(),
        }
    }

    pub fn read(&mut self) {
        let ps2_data: u16 = 0; // TODO

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
        };
    }

    pub fn get_next(&mut self) -> Option<KeyEvent> {
        if self.event_queue_read < self.event_queue_write {
            // TODO: handle overflow
            let (code, state) = self.event_queue[self.event_queue_read];
            self.event_queue_read += 1;
            Some(KeyEvent { code, state })
        } else {
            None
        }
    }
}
