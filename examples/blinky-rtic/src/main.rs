#![no_main]
#![no_std]
mod mono;
use defmt_rtt as _;
use panic_halt as _;

#[rtic::app(device = microbit::hal::pac, peripherals = true, dispatchers = [UART0])]
mod app {
    use super::mono::MonoTimer;
    use microbit::hal::pac::TIMER0;
    use rtic::time::duration::Seconds;

    #[monotonic(binds = TIMER0, default = true)]
    type MyMono = MonoTimer<TIMER0>;

    #[init]
    fn init(ctx: init::Context) -> (init::LateResources, init::Monotonics) {
        let mono = MonoTimer::new(ctx.device.TIMER0);
        tick::spawn().ok();
        (init::LateResources {}, init::Monotonics(mono))
    }

    #[task]
    fn tick(_: tick::Context) {
        defmt::info!("Tick");
        tick::spawn_after(Seconds(1_u32)).ok();
    }
}
