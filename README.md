cargo build --example blinking_led_3
objcopy -O ihex ./target/tc162-htc-none/debug/examples/blinking_led_3.elf output.hex

set active toolchain: 
rustup override set tricore-htc-none-v1.0.0

convert ".elf" file to ".hex" file using objcopy
objcopy -O ihex ./target/tc162-htc-none/debug/examples/blinking_led_3.elf output.hex


C:\HighTec\toolchains\tricore\v7.0.0\bin\clang.exe
C:\HighTec\toolchains\tricore\v4.9.3.0-infineon-1.0\bin\tricore-gcc.exe


This file needed some adaptions to get the project to build properly. After going from error to error for several hours I found all the necessary
flags to build the project and allow for cross compilation.
C:\Users\jeffe\Desktop\Project\tc375-freertos\FreeRTOS-rust\freertos-cargo-build\src\lib.rs


A list of supported devices for embeddde rust: https://github.com/rust-embedded/awesome-embedded-rust#driver-crates


add these lines of code to "FreeRTOS-rust\freertos-cargo-build\src\lib.rs":
 ("tc162-htc-none", _, _, _) => "TriCore",


Rust implementation for different MCU
https://github.com/sheref-sidarous/stm32f3discovery_freertos_rust


C based implementation for TC375
https://github.com/AhmedElghaly/AURIX_TC375_SB-FreeRTOS


Lines 175-197 are responsible for selecting the target port
https://github.com/lobaro/FreeRTOS-rust/blob/b53e66deecc17c0b4c90e0b58b617e2dd8849355/freertos-cargo-build/src/lib.rs

This needs to be added: 

       b.no_default_flags(true);

       // b.flag_if_supported("-O0");
        b.remove_flag("-fpic");
        b.flag("-O0");
        b.flag("-ffunction-sections");
        b.flag("-fdata-sections");
        b.flag("-mcode-pic");
        b.flag("-g");
        b.flag("-fno-omit-frame-pointer");
        //b.flag("-std=c99");
        //b.flag("-std=gnu99");
        b.flag("-std=c11");

        b.flag("-mtc162");