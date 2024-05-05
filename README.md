set active toolchain: 
rustup override set tricore-htc-none-v1.0.0

convert ".elf" file to ".hex" file using objcopy
objcopy -O ihex ./target/tc162-htc-none/debug/examples/blinking_led_3.elf output.hex


C:\HighTec\toolchains\tricore\v7.0.0\bin\clang.exe
C:\HighTec\toolchains\tricore\v4.9.3.0-infineon-1.0\bin\tricore-gcc.exe


add these lines of code to "FreeRTOS-rust\freertos-cargo-build\src\lib.rs":
 ("tc162-htc-none", _, _, _) => "TriCore",


Rust implementation for different MCU
https://github.com/sheref-sidarous/stm32f3discovery_freertos_rust


C based implementation for TC375
https://github.com/AhmedElghaly/AURIX_TC375_SB-FreeRTOS


Lines 175-197 are responsible for selecting the target port
https://github.com/lobaro/FreeRTOS-rust/blob/b53e66deecc17c0b4c90e0b58b617e2dd8849355/freertos-cargo-build/src/lib.rs