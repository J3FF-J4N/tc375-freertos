// #![no_std]
// #![no_main]

// use tc37x_hal::tc3xx_service;

// use tc37x_hal::isr::load_interrupt_table;
// use tc37x_hal::wdtcon::{disable_cpu_watchdog, disable_safety_watchdog};
// use tc3xx_service::asm_calls::enable_interrupts;
// use tc3xx_service::asm_calls::read_cpu_core_id;
// use tc3xx_service::{entry, post_init, pre_init};

// use embedded_hal::prelude::*;
// use embedded_hal::timer::{Cancel, CountDown};
// use tc375_bsp::leds_and_buttons::{Button1, Led1, Led2};
// use tc37x_hal::timer::Timer;

// use freertos_rust::*;

// use tc37xpd::*;

// #[global_allocator]
// static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

// pre_init!(pre_init_fn);

// fn pre_init_fn() {
//     if read_cpu_core_id() == 0 {
//         disable_safety_watchdog();
//     }
//     disable_cpu_watchdog();
// }

// post_init!(post_init_fn);

// fn post_init_fn() {
//     load_interrupt_table();
//     enable_interrupts();
// }

// entry!(main);

// fn main() -> ! {
//     let mut led1 = Led1::new();
//     let mut timer = Timer::new(tc375_bsp::SYSTEM_TIMER_FREQ_HZ);

//     timer.start(500_u32);
//     let mut is_running = true;

//     led1.set_on();

//     loop {
//         if let Ok(_) = timer.wait() {
//             led1.toggle();
//         }
//     }
// }

// fn toggle_timer(timer: &mut Timer, flag: bool) -> bool {
//     if flag {
//         timer.cancel().ok();
//     } else {
//         timer.restart();
//     }
//     !flag
// }

// pub mod uart {

//     use embedded_hal::serial::*;

//     pub struct UART {}

//     impl UART {
//         pub fn new() -> Self {
//             let parts = tc37xpd::ASCLIN3.split();
//             let mut pin = parts.pin02;
//             //Set the turn off bit in register before set the mode to output. This is for safety.
//             pin.set_high().unwrap();
//             //Set mode to output
//             pin.into_output_push_pull();
//             //Configure pad driver
//             call_without_cpu_endinit(|| {
//                 pin.configure_fast_output_pad(MediumDriver);
//                 pin.configure_input_pad(AutomotiveLevel);
//             });
//             Self { pin }
//         }
//     }
// }
