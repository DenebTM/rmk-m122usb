use core::convert::Infallible;
use embassy_rp::{
    gpio::Level,
    pio::{
        Common, Config, Direction as PioDirection, FifoJoin, Instance as PioInstance,
        LoadedProgram, Pio, PioPin, ShiftDirection, StateMachine,
    },
};
use embedded_io_async::{ErrorType, Read};
use fixed::traits::ToFixed;

struct PioPS2RxProgram<'a, PIO: PioInstance> {
    prg: LoadedProgram<'a, PIO>,
}

impl<'a, PIO: PioInstance> PioPS2RxProgram<'a, PIO> {
    /// Load the uart rx program into the given pio
    fn new(common: &mut Common<'a, PIO>) -> Self {
        let prg = pio_proc::pio_asm!(
            r#"
                ; basic 8O1 clocked serial interface for PS/2
                ; does not check start/stop bits nor parity
                ; pin 0 is data, pin 1 is clock

                start:
                    wait 0 pin 1        ; wait for clock to be pulled low
                    set x, 10           ; preload bit counter

                rx_bitloop:
                    wait 0 pin 1        ; wait for rising edge of the clock pin
                    wait 1 pin 1
                    in pins, 1          ; shift data bit into ISR
                    jmp x-- rx_bitloop  ; loop 11 times

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
pub struct PioPs2Rx<'a, PIO: PioInstance> {
    sm_rx: StateMachine<'a, PIO, 0>,
}

impl<'a, PIO: PioInstance> PioPs2Rx<'a, PIO> {
    /// Configure a pio state machine to use the loaded rx program.
    pub fn new(pio: Pio<'static, PIO>, data_pin: impl PioPin, clk_pin: impl PioPin) -> Self {
        let Pio {
            mut common,
            sm0: mut sm_rx,
            ..
        } = pio;

        let program = PioPS2RxProgram::new(&mut common);

        let mut cfg = Config::default();
        cfg.use_program(&program.prg, &[]);

        let data_pin = common.make_pio_pin(data_pin);
        let clk_pin = common.make_pio_pin(clk_pin);
        sm_rx.set_pins(Level::High, &[&data_pin, &clk_pin]);
        cfg.set_in_pins(&[&data_pin, &clk_pin]);
        cfg.set_jmp_pin(&clk_pin);
        sm_rx.set_pin_dirs(PioDirection::In, &[&data_pin, &clk_pin]);

        cfg.clock_divider = 1.to_fixed();
        cfg.shift_in.auto_fill = false;
        cfg.shift_in.direction = ShiftDirection::Right;
        cfg.shift_in.threshold = 32;
        cfg.fifo_join = FifoJoin::RxOnly;
        sm_rx.set_config(&cfg);
        sm_rx.set_enable(true);

        Self { sm_rx }
    }

    /// Wait for PS/2 packet (8 bits + start/stop/parity)
    pub async fn read_packet(&mut self) -> u16 {
        self.sm_rx.rx().wait_pull().await as u16
    }
}

impl<PIO: PioInstance> ErrorType for PioPs2Rx<'_, PIO> {
    type Error = Infallible;
}

//impl<PIO: PioInstance> Read for PioPs2Rx<'_, PIO> {
//    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Infallible> {
//        let mut i = 0;
//        while (i + 1) < buf.len() {
//            let packet = self.read_packet().await;
//            buf[i] = (packet & 0xFF) as u8;
//            buf[i + 1] = (packet >> 8) as u8;
//            i += 2;
//        }
//        Ok(i)
//    }
//}
