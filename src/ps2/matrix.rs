use defmt::error;
use embassy_rp::gpio::Input;
use pc_keyboard::KeyCode;
use rmk::{
    keyboard::{key_event_channel, KeyEvent},
    matrix::{KeyState, MatrixTrait},
};

use super::port::PS2Port;

pub struct PS2Matrix<const ROW: usize, const COL: usize> {
    port: PS2Port,
    matrix: [[KeyState; COL]; ROW],
}

impl<const ROW: usize, const COL: usize> PS2Matrix<ROW, COL> {
    pub fn new(clk_pin: Input<'static>, data_pin: Input<'static>) -> Self {
        Self {
            port: PS2Port::new(clk_pin, data_pin),
            matrix: [[KeyState { pressed: false }; COL]; ROW],
        }
    }
}

fn keycode_to_pos(keycode: KeyCode) -> (usize, usize) {
    // TODO
    match keycode {
        _ => (0, 0),
    }
}

impl<const ROW: usize, const COL: usize> MatrixTrait for PS2Matrix<ROW, COL> {
    const ROW: usize = ROW;
    const COL: usize = COL;

    fn get_row_num(&self) -> usize {
        return ROW;
    }

    fn get_col_num(&self) -> usize {
        return COL;
    }

    async fn scan(&mut self) {
        while let Some(pc_keyboard::KeyEvent { code, state }) = self.port.get_next() {
            defmt::debug!("Processing PS/2 key event");
            let (row, col) = keycode_to_pos(code);

            self.matrix[row][col] = KeyState {
                pressed: state == pc_keyboard::KeyState::Down,
            };

            let send_re = key_event_channel.try_send(KeyEvent {
                row: row as u8,
                col: col as u8,
                pressed: self.matrix[row][col].pressed,
            });
            if send_re.is_err() {
                error!("Failed to send key event: key event channel full");
            }
        }
    }

    fn update_key_state(&mut self, row: usize, col: usize, f: impl FnOnce(&mut KeyState)) {
        f(&mut self.matrix[row][col])
    }

    fn get_key_state(&mut self, row: usize, col: usize) -> KeyState {
        self.matrix[row][col]
    }
}
