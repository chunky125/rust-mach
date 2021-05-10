mod miniuart;

use core::ptr::replace;
use miniuart::MiniUART;

struct Peripherals {
    miniuart: Option<MiniUART>,
}

impl Peripherals {

    fn take_miniuart(&mut self) -> MiniUART {

        let p = replace(&mut self.miniuart, None);

        p.unwrap();
    }
}

static mut PERIPHERALS: Peripherals = Peripherals {
    miniuart: Some(MiniUART),
};
