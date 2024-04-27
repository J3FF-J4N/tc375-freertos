#![no_std]

/// Default frequency of system timer of Aurix TC375 Lite Kit in Hz.
pub const SYSTEM_TIMER_FREQ_HZ: u32 = 50_000_000;

/// This module contains structures handling the leds and button of Aurix TC375 Lite Kit.
///
/// The board has two leds wired into Pin5 and Pin6 of Port_00, and one button wired into Pin7 of Port_00. This module provides separate structures for the two led
/// and one structure for handling the one button.
///
/// #### Examples
/// ``` rust
/// use mod tx375-bsp::leds_and_buttons::{Led1, Led2, Button1};
/// let mut led1 = Led1::new();
/// let button1 = Button1::new();
/// if button1.is_pressed() {
///     led1.toggle();
/// }
/// ```
pub mod leds_and_buttons {
    use embedded_hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};
    use tc37x_hal::digital::v2::{
        self as gpio,
        pad_driver_codes::{FastPadOutputCodes::MediumDriver, InputCodes::AutomotiveLevel},
        GpioExt,
    };
    use tc37x_hal::tc37xpd;
    use tc37x_hal::wdtcon::call_without_cpu_endinit;

    /// Handles led Led1 wired to Pin5 of Port_00.
    ///
    /// Led1 is implemented by using [`tc37x_hal::digital::v2::port_00::Pin05`] structure defined in HAL library. It provides [`Led1::set_on`],[`Led1::set_off`],[`Led1::toggle`]
    /// methods.
    ///
    /// #### Examples:
    /// ``` rust
    /// use mod tx375_bsp::leds_and_buttons::Led1;
    /// let mut led1 = Led1::new();
    /// led1.toggle();
    /// ```
    pub struct Led1 {
        pin: gpio::p10::Pin02,
    }

    impl Led1 {
        /// Configure Pin05 of Port_00 to output push-pull mode and return a new Led1 structure.
        pub fn new() -> Self {
            let parts = tc37xpd::P10.split();
            let mut pin = parts.pin02;
            //Set the turn off bit in register before set the mode to output. This is for safety.
            pin.set_high().unwrap();
            //Set mode to output
            pin.into_output_push_pull();
            //Configure pad driver
            call_without_cpu_endinit(|| {
                pin.configure_fast_output_pad(MediumDriver);
                pin.configure_input_pad(AutomotiveLevel);
            });
            Self { pin }
        }

        /// Toggles Led1.
        #[inline(always)]
        pub fn toggle(&mut self) {
            self.pin.toggle().unwrap();
        }

        /// Sets on Led1.
        #[inline(always)]
        pub fn set_on(&mut self) {
            self.pin.set_low().unwrap();
        }

        /// Sets off Led1.
        #[inline(always)]
        pub fn set_off(&mut self) {
            self.pin.set_high().unwrap();
        }
    }

    impl Default for Led1 {
        fn default() -> Self {
            Led1::new()
        }
    }

    /// Handles led Led2 wired to Pin6 of Port_00.
    ///
    /// Led2 is implemented by using [`tc37x_hal::digital::v2::port_00::Pin06`] structure defined in HAL library. It provides [`Led2::set_on`],[`Led2::set_off`],[`Led2::toggle`]
    /// methods.
    ///
    /// #### Examples:
    /// ``` rust
    /// use mod tx375-bsp::leds_and_buttons::Led2;
    /// let mut led2 = Led2::new();
    /// led2.toggle();
    /// ```
    pub struct Led2 {
        pin: gpio::p10::Pin02,
    }

    impl Led2 {
        /// Configure Pin06 of Port_00 to output push-pull mode and return a new Led2 structure.
        pub fn new() -> Self {
            let parts = tc37xpd::P10.split();
            let mut pin = parts.pin02;
            //Set the turn off bit in register before set the mode to output. This is for safety.
            pin.set_high().unwrap();
            //Set mode to output
            pin.into_output_push_pull();
            //Configure pad driver
            call_without_cpu_endinit(|| {
                pin.configure_fast_output_pad(MediumDriver);
                pin.configure_input_pad(AutomotiveLevel);
            });
            Self { pin }
        }

        /// Toggles Led2.
        #[inline(always)]
        pub fn toggle(&mut self) {
            self.pin.toggle().unwrap();
        }

        /// Sets on Led2.
        #[inline(always)]
        pub fn set_on(&mut self) {
            self.pin.set_low().unwrap();
        }

        /// Sets off Led2.
        #[inline(always)]
        pub fn set_off(&mut self) {
            self.pin.set_high().unwrap();
        }
    }

    impl Default for Led2 {
        fn default() -> Self {
            Led2::new()
        }
    }

    /// Handles button Button1 wired to Pin7 of Port_00.
    ///
    /// Button1 is implemented by using [`tc37x_hal::digital::v2::port_00::Pin07`] structure defined in HAL library. It provides a [`Button1::is_pressed`] method.
    ///
    /// #### Examples
    /// ``` rust
    /// use mod tx375-bsp::leds_and_buttons::Button1;
    /// let button1 = Button1::new();
    /// loop {
    ///     if button1.is_pressed() {
    ///         ()
    ///     }
    /// }
    /// ```  
    pub struct Button1 {
        pin: gpio::p10::Pin07,
    }

    impl Button1 {
        /// Configures Pin07 of Port_00 to pull up input mode and returns a Button1 structure.
        pub fn new() -> Self {
            let parts = tc37xpd::P10.split();
            let pin = parts.pin07;
            pin.into_pull_up_input();
            Self { pin }
        }

        /// Return true if Button1 is pressed.
        #[inline(always)]
        pub fn is_pressed(&self) -> bool {
            self.pin.is_low().unwrap()
        }
    }

    impl Default for Button1 {
        fn default() -> Self {
            Self::new()
        }
    }
}
