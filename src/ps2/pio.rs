use core::convert::Infallible;
use embassy_rp::{
    gpio::Level,
    pio::{
        Common, Config, Direction as PioDirection, FifoJoin, Instance, LoadedProgram, PioPin,
        ShiftDirection, StateMachine,
    },
};
use embedded_io_async::{ErrorType, Read};
use fixed::traits::ToFixed;

pub struct PioPS2RxProgram<'a, PIO: Instance> {
    prg: LoadedProgram<'a, PIO>,
}

impl<'a, PIO: Instance> PioPS2RxProgram<'a, PIO> {
    /// Load the uart rx program into the given pio
    pub fn new(common: &mut Common<'a, PIO>) -> Self {
        let prg = pio_proc::pio_asm!(
            r#"
                ; basic 8O1 clocked serial interface for PS/2

                start:
                    wait 0 pin 0        ; wait for clock to be pulled low
                    set x, 10           ; preload bit counter

                rx_bitloop:
                    wait 1 pin 0        ; wait for rising edge of the clock pin
                    in pins, 1          ; shift data bit into ISR
                    jmp x-- rx_bitloop  ; loop 11 times

                    wait 1 pin 0        ; wait for clock pin to return to idle state

                rx_stop:
                    in null 21
                    push
            "#
        );

        let prg = common.load_program(&prg.program);

        Self { prg }
    }
}

/// PIO backed Uart reciever
pub struct PioPs2Rx<'a, PIO: Instance, const SM: usize> {
    sm_rx: StateMachine<'a, PIO, SM>,
}

impl<'a, PIO: Instance, const SM: usize> PioPs2Rx<'a, PIO, SM> {
    /// Configure a pio state machine to use the loaded rx program.
    pub fn new(
        common: &mut Common<'a, PIO>,
        mut sm_rx: StateMachine<'a, PIO, SM>,
        clk_pin: impl PioPin,
        data_pin: impl PioPin,
        program: &PioPS2RxProgram<'a, PIO>,
    ) -> Self {
        let mut cfg = Config::default();
        cfg.use_program(&program.prg, &[]);

        let clk_pin = common.make_pio_pin(clk_pin);
        let data_pin = common.make_pio_pin(data_pin);
        sm_rx.set_pins(Level::High, &[&clk_pin, &data_pin]);
        cfg.set_in_pins(&[&clk_pin, &data_pin]);
        cfg.set_jmp_pin(&clk_pin);
        sm_rx.set_pin_dirs(PioDirection::In, &[&clk_pin, &data_pin]);

        cfg.clock_divider = 1.to_fixed();
        cfg.shift_in.auto_fill = false;
        cfg.shift_in.direction = ShiftDirection::Right;
        cfg.shift_in.threshold = 32;
        cfg.fifo_join = FifoJoin::RxOnly;
        sm_rx.set_config(&cfg);
        sm_rx.set_enable(true);

        Self { sm_rx }
    }

    /// Wait for a single u8
    pub async fn read_u8(&mut self) -> u8 {
        self.sm_rx.rx().wait_pull().await as u8
    }
}

impl<PIO: Instance, const SM: usize> ErrorType for PioPs2Rx<'_, PIO, SM> {
    type Error = Infallible;
}

impl<PIO: Instance, const SM: usize> Read for PioPs2Rx<'_, PIO, SM> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Infallible> {
        let mut i = 0;
        while i < buf.len() {
            buf[i] = self.read_u8().await;
            i += 1;
        }
        Ok(i)
    }
}
