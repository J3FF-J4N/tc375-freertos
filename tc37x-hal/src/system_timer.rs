use tc37xpd::{stm0::Stm0, stm1::Stm1, stm2::Stm2, STM0, STM1, STM2};
use tc3xx_service::asm_calls::read_cpu_core_id;
use void::Void;

/// Stores one of the three system timers.
pub enum SystemTimers {
    Timer0(Stm0),
    Timer1(Stm1),
    Timer2(Stm2),
}

impl SystemTimers {
    /// Creates the SystemTimers variant with the system timer struct of the current CPU core.
    pub fn new() -> Self {
        match read_cpu_core_id() {
            0 => SystemTimers::Timer0(STM0),
            1 => SystemTimers::Timer1(STM1),
            _ => SystemTimers::Timer2(STM2),
        }
    }

    /// Returns the inner system timer struct.
    pub fn get_timer(&self) -> &dyn SystemTimerFunctions {
        match self {
            SystemTimers::Timer0(timer) => timer,
            SystemTimers::Timer1(timer) => timer,
            SystemTimers::Timer2(timer) => timer,
        }
    }
}

impl Default for SystemTimers {
    fn default() -> Self {
        Self::new()
    }
}

/// Supertrait for the traits related to system timers, this marker trait enables the definition of
/// a single get_timer(&self) -> &dyn SystemTimerFunctions function instead of two similar, separate
/// funcitons for the DelayFunctions and the TimerFunctions traits
pub trait SystemTimerFunctions: DelayFunctions + TimerFunctions {}
impl SystemTimerFunctions for Stm0 {}
impl SystemTimerFunctions for Stm1 {}
impl SystemTimerFunctions for Stm2 {}

/// Function declarations for [`embedded_hal::blocking::delay`] associated with system timer types.
pub trait DelayFunctions {
    fn delay(&self, ticks: u64);
    fn read_ticks(&self) -> u64;
}

/// Function declarations for [`embedded_hal::timer`] associated with system timer types.
pub struct AlreadyCancelled;
pub trait TimerFunctions {
    fn cancel(&self) -> Result<(), AlreadyCancelled>;
    fn init(&self);
    fn restart(&self);
    fn start(&self, ticks: u32);
    fn start_internal(&self);
    fn wait(&self) -> nb::Result<(), Void>;
}
