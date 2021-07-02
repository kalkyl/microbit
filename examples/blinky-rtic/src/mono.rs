use rtic_monotonic::{embedded_time, Clock, Fraction, Instant, Monotonic};

pub struct MonoTimer(microbit::hal::pac::TIMER0);

impl MonoTimer {
    pub fn new(timer: microbit::hal::pac::TIMER0) -> Self {
        timer.prescaler.write(
            |w| unsafe { w.prescaler().bits(4) }, // 1 MHz
        );
        timer.bitmode.write(|w| w.bitmode()._32bit());
        MonoTimer(timer)
    }
}

impl Clock for MonoTimer {
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000);
    type T = u32;

    #[inline(always)]
    fn try_now(&self) -> Result<Instant<Self>, embedded_time::clock::Error> {
        self.0.tasks_capture[1].write(|w| unsafe { w.bits(1) }); 
        Ok(Instant::new(self.0.cc[1].read().bits()))
    }
}

impl Monotonic for MonoTimer {
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
