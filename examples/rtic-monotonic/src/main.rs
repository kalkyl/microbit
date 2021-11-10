#![no_main]
#![no_std]
mod mono;
use defmt_rtt as _;
use panic_halt as _;

#[rtic::app(device = microbit::hal::pac, peripherals = true, dispatchers = [UART0])]
mod app {
    use super::mono::{ExtU32, MonoTimer};
    use microbit::hal::pac::TIMER0;

    #[monotonic(binds = TIMER0, default = true)]
    type MyMono = MonoTimer<TIMER0>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mono = MonoTimer::new(ctx.device.TIMER0);
        tick::spawn().ok();
        (Shared {}, Local {}, init::Monotonics(mono))
    }

    #[task]
    fn tick(_: tick::Context) {
        defmt::info!("Tick!");
        tick::spawn_after(1.secs()).ok();
    }
}
