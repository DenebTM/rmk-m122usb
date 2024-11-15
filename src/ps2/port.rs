type PS2Scancode = u16;

pub(crate) struct PS2Port {
    clk_pin: u16,
    data_pin: u16,

    scancode_queue: [PS2Scancode; 256],
    scancode_queue_write: usize,
    scancode_queue_read: usize,
}

impl PS2Port {
    pub fn new(clk_pin: u16, data_pin: u16) -> Self {
        Self {
            clk_pin,
            data_pin,

            scancode_queue: [0; 256],
            scancode_queue_write: 0,
            scancode_queue_read: 0
        }
    }

    pub fn read(&mut self) {
        self.scancode_queue[self.scancode_queue_write] = 0;
        self.scancode_queue_write += 1;
    }

    pub fn get_next(mut self) -> Option<PS2Scancode> {
        if self.scancode_queue_read < self.scancode_queue_write {
            self.scancode_queue_read += 1;
            Some(self.scancode_queue[self.scancode_queue_read - 1])
        } else {
            None
        }
    }
}
