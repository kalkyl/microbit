#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;



use rtic_monotonic::{embedded_time, Clock, Fraction, Instant, Monotonic};

pub struct MyTimer(microbit::hal::pac::TIMER0);

impl MyTimer {
    fn new(timer: microbit::hal::pac::TIMER0) -> Self {
        timer.prescaler.write(
            |w| unsafe { w.prescaler().bits(4) }, // 1 MHz
        );
        timer.bitmode.write(|w| w.bitmode()._32bit());
        MyTimer(timer)
    }
}

impl Clock for MyTimer {
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000);
    type T = u32;

    #[inline(always)]
    fn try_now(&self) -> Result<Instant<Self>, embedded_time::clock::Error> {
        self.0.tasks_capture[1].write(|w| unsafe { w.bits(1) }); 
        Ok(Instant::new(self.0.cc[1].read().bits()))
    }
}

impl Monotonic for MyTimer {
    unsafe fn reset(&mut self) { 
        self.0.intenset.modify(|_, w| w.compare0().set());
        self.0.tasks_clear.write(|w| unsafe { w.bits(1) });
        self.0.tasks_start.write(|w| unsafe { w.bits(1) });
    }

    fn set_compare(&mut self, instant: &Instant<Self>) {
        #[cfg(feature = "v1")]
        self.0.cc[0].write(|w| unsafe {w.bits(*instant.duration_since_epoch().integer())});
        #[cfg(feature = "v2")]
        self.0.cc[0].write(|w| unsafe {w.cc().bits(*instant.duration_since_epoch().integer())}); 
    }

    fn clear_compare_flag(&mut self) {
        self.0.events_compare[0].write(|w| w);
    }
}

#[rtic::app(device = microbit::hal::pac, peripherals = true, dispatchers = [GPIOTE])]
mod app {
    use super::MyTimer;
    use embedded_hal::{digital::v2::OutputPin};
    use rtic::time::duration::Seconds;
    use microbit::hal::{
        gpio::{p0, Level},
        timer::{Timer, Periodic},
        pac::TIMER0
    };


    #[monotonic(binds = TIMER0, default = true)]
    type MyMono = MyTimer; // 16 MHz

    #[resources]
    struct Resources {
        // led: PA5<Output<PushPull>>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (init::LateResources, init::Monotonics) {

        let mut mono = MyTimer::new(ctx.device.TIMER0);

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


