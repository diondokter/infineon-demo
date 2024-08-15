#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _, psoc6_02_pac as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello!");
    loop {
        cortex_m::asm::bkpt();
    }
}
