use std::env;

use freertos_cargo_build::*;

/*


*/

fn main() {
    env::set_var(
        "AR",
        "C:\\HighTec\\toolchains\\tricore\\v4.9.3.0-infineon-1.0\\bin\\tricore-ar.exe",
    );

    let mut b = freertos_cargo_build::Builder::new();

    // Path to FreeRTOS kernel or set ENV "FREERTOS_SRC" instead
    b.freertos("FreeRTOS-Kernel/");
    b.freertos_config("src"); // Location of `FreeRTOSConfig.h`
    b.freertos_port("TriCore"); // Port dir relativ to 'FreeRTOS-Kernel/portable'
    b.heap("heap_4.c"); // Set the heap_?.c allocator to use from
                        //'FreeRTOS-Kernel/portable/MemMang' (Default: heap_4.c)

    b.compile().unwrap_or_else(|e| panic!("{}", e.to_string()));
}
