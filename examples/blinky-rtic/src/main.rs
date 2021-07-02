#![no_main]
#![no_std]
mod mono;

use defmt_rtt as _;
use panic_halt as _;

#[rtic::app(device = microbit::hal::pac, peripherals = true, dispatchers = [GPIOTE])]
mod app {
    use super::mono::MonoTimer;
    use embedded_hal::{digital::v2::OutputPin};
    use rtic::time::duration::Seconds;
    use microbit::hal::{
        gpio::{p0, Level},
        timer::{Timer, Periodic},
        pac::TIMER0
    };


    #[monotonic(binds = TIMER0, default = true)]
    type MyMono = MonoTimer; // 16 MHz

    #[resources]
    struct Resources {
        // led: PA5<Output<PushPull>>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (init::LateResources, init::Monotonics) {

        let mut mono = MonoTimer::new(ctx.device.TIMER0);

        defmt::info!("Hello world!");
        blink::spawn_after(Seconds(1_u32)).ok();
        (init::LateResources { }, init::Monotonics(mono))
    }

    #[task(resources = [])]
    fn blink(mut ctx: blink::Context) {
        // ctx.resources.led.lock(|l| l.toggle().ok());
        defmt::info!("Blink!");
        blink::spawn_after(Seconds(1_u32)).ok();
    }
}


