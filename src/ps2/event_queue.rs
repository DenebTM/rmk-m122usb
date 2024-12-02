pub(crate) struct EventQueue<T: Copy, const S: usize> {
    write: usize,
    read: usize,

    events: [T; S],
}

impl<T: Copy, const S: usize> EventQueue<T, S> {
    pub(super) fn new(init: T) -> Self {
        Self {
            events: [init; S],
            write: 0,
            read: 0,
        }
    }

    pub(super) fn push(&mut self, event: T) -> Result<(), ()> {
        if (self.write + 1) % S == self.read {
            return Err(());
        }

        self.events[self.write] = event;
        self.write = (self.write + 1) % S;

        Ok(())
    }

    pub(super) fn pop(&mut self) -> Option<T> {
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
