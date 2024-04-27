rustup override set tricore-htc-none-v1.0.0
objcopy -O ihex ./target/tc162-htc-none/debug/examples/blinking_led_3.elf output.hex

# TC375 Lite Kit BSP

The TC375 Lite Kit BSP is a small functional project designed to simplify an evaluation phase of the
TC375 microcontroller architecture. It comes with necessary low-level functions like a startup code,
minimalistic hardware abstraction, and predefined memory partitioning. On top of this low-level
implementation, it provides hardware abstractions and reference application examples running on one core.

This project contains libraries and examples for the 
[Aurix TC375 Lite Kit](https://www.infineon.com/cms/en/product/promopages/AURIX-microcontroller-boards/low-cost-arduino-kits/aurix-tc375-lite-kit/) written in Rust.
Each library implements a layer of an embedded application: 
* [Examples/Application](##examplesapplication)
* [Board Support Package (BSP)](#board-support-package-bsp)
* [TC37X Hardware Abstraction Layer (tc37x-hal)](#tc37x-hardware-abstraction-layer-tc37x-hal)
* [TC37XPD Peripheral Access Crate (tc37xpd)](#tc37xpd-peripheral-access-crate-tc37xpd)
* [TC3XX runtime library (tc3xx-service)](#tc3xx-runtime-library-tc3xx-service)

### External dependencies

The generated PAC library depends on [vcell](https://crates.io/crates/vcell) which is published with both 
[Apache](https://www.apache.org/licenses/LICENSE-2.0) and [MIT](https://opensource.org/license/mit/) lincenses.

tc37x-hal and tc375-bsp libraries depend on [embedded-hal](https://crates.io/crates/embedded-hal) and [nb](https://crates.io/crates/nb) crates which are published with 
both [Apache](https://www.apache.org/licenses/LICENSE-2.0) and [MIT](https://opensource.org/license/mit/) lincenses.

The embedded-hal crate has more simple and development dependencies. Check the crate's [page](https://crates.io/crates/embedded-hal/0.2.7/dependencies).

## Details of layers

### Examples/Application

This workspace contains three example programs. Each example can be compiled to an elf file that can be loaded to the board.

#### Example blinking_led_1

Blinking_led_1 example demonstrates the use of the [TC37XPD Peripheral Access Crate (tc37xpd)](#tc37xpd-peripheral-access-crate-tc37xpd)
without the other abstraction layers. The example manipulates the memory mapped I/O registers of the microcontroller directly.

#### Example blinking_led_2

Blinking_led_2 example demonstrates the use of the [Board Support Package (BSP)](#board-support-package-bsp) and the 
GPIO and Delay traits implemented in the [TC37X Hardware Abstraction Layer (tc37x-hal)](#tc37x-hardware-abstraction-layer-tc37x-hal).

The example blinks the leds and handles input from Button1 using the BSP and HAL abstraction layers. 

#### Example blinking_led_3

Blinking_led_3 example is similar to blinking_led_2. It demonstrates using the Timer trait implemented
in the [TC37X Hardware Abstraction Layer (tc37x-hal)](#tc37x-hardware-abstraction-layer-tc37x-hal).

### Board Support Package (BSP)

BSP contains modules to access concrete peripherals of the supported board. In this case support is limited to the leds and button of Lite Kit. 
BSP also has three examples. One demonstrates the use of PAC library, the others demonstrate using GPIO and Timer/Delay traits from HAL library.

### TC37X Hardware Abstraction Layer (tc37x-hal)

A Hardware Abstraction Layer (HAL) for TC37X microcontrollers. Implements watchdog control, interrupt table loading and `embedded-hal` traits.
https://crates.io/crates/embedded-hal[embedded-hal] is a crate containing traits which define interfaces for microcontroller peripherals.
This HAL implementation contains only the Delay, Timer and GPIO traits.

### TC37XPD Peripheral Access Crate (tc37xpd)

TC37X Peripheral Access Crate from Infineon's SVD file.

### TC3XX runtime library (tc3xx-service)

This library contains startup code written in Rust and inlined assembly startup code. The library also contains a linker script with predefined 
memory partitioning and some functions to turn on/off watchdog and endinit bit protection.

## Building the project

Workspace has a `.cargo/config` file which defines target, compiler and linker script for building project.

To build the project with all examples, run

``` bash
cargo build --release --examples
```

### Building documentation

To create documentation run `cargo doc`:

``` bash
cargo doc  --examples --open
```
Cargo does not generate an index page for the workspace by default. Use the following parameters to generate one:
``` bash
RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo doc --examples --open
```




