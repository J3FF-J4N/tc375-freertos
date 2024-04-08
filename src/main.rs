//! # Example project #1
//!
//! The first example application is part of [`tc375_bsp`] library. It demonstrates the usage of peripheral access [`tc37x_pac`] library.
//! The application directly reads and writes memory mapped I/O registers.
//!
//! The function of this example is blinking the leds, the embedded 'hello world'.
//! First led is blinked in an endless loop and the other is blinked from a system timer interrupt routine.
//! Example runs on core CPU0. There are no more threads running on other cores.
//!
//! The example can be compiled to an elf file, which can be flashed to the Aurix TC375 Lite Kit board, with a hardware debugger or the Infineon MemTool.
//!
//! ## Entry points
//!
//! The executable has four entry points. These functions are linked together with the init script of [`tc37x_rt`] runtime library.
//!
//! 1. Pre init function
//! 2. Post init function
//! 3. Main function
//! 4. Interrupt handler 2
//!
//!
//! ## Pre init function
//!
//! Pre init function ( `pre_init_fn` ) does the following:
//! 1. turns off system safety watchdog,
//! 2. turns off CPU watchdog.
//!
//! ## Post Init function
//!
//! Post init function ( `post_init_fn` ) does the following:
//! 1. loads the interrupt table for the CPU in a special core register.
//!
//! ## Main Function
//!
//! The `main` function follows through the following steps:
//! 1. Initializes Led1 (Pin05 of Port_00) and Led2 (Pin06 of Port_00)
//! 2. Initializes system timer 0
//!     - configures system timer,
//!     - sets `interrupt_handler` function to handle this interrupt
//!     - and enables interrupt for timer
//! 3. In an endless loop:
//!     - delays for .5 secs and
//!     - blinks Led1
//!
//! ## Interrupt handler 2
//!
//! Private `interrupt_handler` function attains:
//! 1. toggles Led2 and
//! 2. reenables timer interrupt.
//! 
//! 

//File at: target/tc162-htc-none/release/examples/blinking_led_1.elf

#![no_std]
#![no_main]

use tc37x_pac::interrupt;
use tc37x_rt::asm_calls::{enable_interrupts, read_cpu_core_id};
use tc37x_rt::isr::load_interrupt_table;
use tc37x_rt::wdtcon::{call_without_cpu_endinit, disable_cpu_watchdog, disable_safety_watchdog};
use tc37x_rt::{entry, post_init, pre_init};

/// Ticks per sec.
const SYSTEM_TIMER_FREQ_HZ: u32 = 50_000_000_u32;

#[allow(dead_code)]
enum BoardLedAction {
    NoChange,
    TurnOff,
    TurnOn,
    Toggle,
}

impl BoardLedAction {
    pub fn as_mask(&self) -> u32 {
        match self {
            BoardLedAction::NoChange => 0,
            BoardLedAction::TurnOff => 1,
            BoardLedAction::TurnOn => 1 << 16,
            BoardLedAction::Toggle => (1 << 16) | 1,
        }
    }
}

// LED GPIO for AURIX lite Kit
enum BoardLed {
    Led1 = 5,
    Led2 = 6,
}

impl BoardLed {
    fn set_led(self, action: BoardLedAction) {
        let p = unsafe { tc37x_pac::Peripherals::steal() };
        let p00 = p.PORT_00;

        let led_no = self as u32;
        let mask = action.as_mask();
        let mask = mask << led_no;
        p00.omr.write(|w| unsafe { w.bits(mask) });
    }

    fn toggle_led(self) {
        self.set_led(BoardLedAction::Toggle);
    }

    fn set_on(self) {
        self.set_led(BoardLedAction::TurnOn);
    }
}

pre_init!(pre_init_fn);
post_init!(post_init_fn);
entry!(main);
interrupt!(__INTERRUPT_HANDLER_2, interrupt_handler);

fn pre_init_fn() {
    if read_cpu_core_id() == 0 {
        disable_safety_watchdog();
    }
    disable_cpu_watchdog();
}

fn post_init_fn() {
    load_interrupt_table();
}

fn main() -> ! {
    // Init LED port
    board_led_init();

    create_timer_interrupt();
    enable_interrupts();

    BoardLed::Led1.set_on();
    loop {
        delay();
        BoardLed::Led1.toggle_led();
    }
}

/// Init board.
fn board_led_init() {
    let p = unsafe { tc37x_pac::Peripherals::steal() };
    let p00 = p.PORT_00;
    set_port_mode_output(&p00);
    set_port_pad_driver_mode(&p00);
}

fn set_port_mode_output(p00: &tc37x_pac::PORT_00) {
    // Clear pin
    p00.omr.write(|w| w.ps5().set_bit());
    // Set to output
    p00.iocr4.modify(|_, w| w.pc5().variant(0x10));

    p00.omr.write(|w| w.ps6().set_bit());
    p00.iocr4.modify(|_, w| w.pc6().variant(0x10));
}

fn set_port_pad_driver_mode(p00: &tc37x_pac::PORT_00) {
    call_without_cpu_endinit(|| {
        // Pad drivers in medium mode, automotive drive strength
        #[rustfmt::skip]
        p00.pdr0.modify(|_, w| { w
            .pd5().variant(2).pl5().variant(0)
            .pd6().variant(2).pl6().variant(0)
        });
        // See Aurix TC3XX target specification, section of General Purpose IO/registers
    });
}

fn delay() {
    let p = unsafe { tc37x_pac::Peripherals::steal() };
    let stm0 = p.STM0;
    let time = stm0.tim1.read().bits();
    let mut nt = time;
    // Sleep for .5 sec, reading tim1 (not tim0) register which is right padded with 4 bits
    while nt.wrapping_sub(time) < (SYSTEM_TIMER_FREQ_HZ >> 4) / 2 {
        nt = stm0.tim1.read().bits();
    }
}

fn create_timer_interrupt() {
    init_timer_compare_registers();
    enable_timer0_interrupt();
}

fn init_timer_compare_registers() {
    let p = unsafe { tc37x_pac::Peripherals::steal() };

    // Setup the system timer
    let stm0 = p.STM0;
    let time = stm0.tim0.read().bits();
    let next_time = time.wrapping_add(SYSTEM_TIMER_FREQ_HZ);
    // See Aurix TC3XX target specification, section of System timers
    // Set compare match control register
    stm0.cmcon.modify(|_, w| w.msize0().variant(31));
    // Set compare
    stm0.cmp0.write(|w| w.cmpval().variant(next_time));
    // Interrupt set/clear register - reset compare register
    stm0.iscr.write(|w| w.cmp0irr().set_bit());
    // Interrupt control register - enable compare
    stm0.icr.write(|w| w.cmp0en().set_bit());
}

fn enable_timer0_interrupt() {
    let p = unsafe { tc37x_pac::Peripherals::steal() };
    let src = p.SRC;
    // See 'Interrupt Router section' in specification
    src.stm0sr0
        .modify(|_, w| w.tos().variant(0).srpn().variant(2).sre().set_bit());
}

fn interrupt_handler() {
    BoardLed::Led2.toggle_led();

    let p = unsafe { tc37x_pac::Peripherals::steal() };
    let stm0 = p.STM0;
    let time = stm0.cmp0.read().bits();
    let next_time = time.wrapping_add(SYSTEM_TIMER_FREQ_HZ);
    stm0.cmp0.write(|w| unsafe { w.bits(next_time) });
    stm0.iscr.write(|w| w.cmp0irr().set_bit());
}
