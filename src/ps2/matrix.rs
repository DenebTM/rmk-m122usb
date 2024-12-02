use defmt::{error, info};
use embassy_rp::pio::Instance as PioInstance;
use rmk::{
    keyboard::{key_event_channel, KeyEvent},
    matrix::{KeyState, MatrixTrait},
};

use super::port::PS2Port;

pub struct PS2Matrix<const ROW: usize, const COL: usize, PIO: PioInstance + 'static> {
    port: &'static PS2Port<PIO>,
    matrix: [[KeyState; COL]; ROW],
}

impl<const ROW: usize, const COL: usize, PIO: PioInstance + 'static> PS2Matrix<ROW, COL, PIO> {
    pub fn new(port: &'static PS2Port<PIO>) -> Self {
        Self {
            port,
            matrix: [[KeyState { pressed: false }; COL]; ROW],
        }
    }
}

impl<const ROW: usize, const COL: usize, PIO: PioInstance + 'static> MatrixTrait
    for PS2Matrix<ROW, COL, PIO>
{
    const ROW: usize = ROW;
    const COL: usize = COL;

    fn get_row_num(&self) -> usize {
        return ROW;
    }

    fn get_col_num(&self) -> usize {
        return COL;
    }

    async fn scan(&mut self) {
        info!("PS/2 matrix scanning task");

        loop {
            while let Some(key_event@KeyEvent { row, col, pressed }) = self.port.pop_event().await {
                defmt::debug!("Processing PS/2 key event");

                self.matrix[row as usize][col as usize] = KeyState {
                    pressed
                };

                let send_re = key_event_channel.try_send(key_event);
                if send_re.is_err() {
                    error!("Failed to send key event: key event channel full");
                }
            }

            embassy_time::Timer::after_micros(100).await;
        }
    }

    fn update_key_state(&mut self, row: usize, col: usize, f: impl FnOnce(&mut KeyState)) {
        f(&mut self.matrix[row][col])
    }

    fn get_key_state(&mut self, row: usize, col: usize) -> KeyState {
        self.matrix[row][col]
    }
}
