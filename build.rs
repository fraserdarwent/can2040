extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/can2040.c");

    cmake::Config::new("lib/pico-sdk")
        .no_build_target(true)
        .always_configure(true)
        .out_dir("lib/pico-sdk")
        .build();

    cc::Build::new()
        .file("src/can2040.c")
        .include("lib/pico-sdk/build/generated/pico_base/")
        .include("lib/pico-sdk/src/common/pico_base/include/")
        .include("lib/pico-sdk/src/rp2_common/hardware_base/include/")
        .include("lib/pico-sdk/src/rp2_common/cmsis/stub/CMSIS/Core/Include/")
        .include("lib/pico-sdk/src/rp2_common/cmsis/stub/CMSIS/Device/RaspberryPi/RP2040/Include/")
        .include("lib/pico-sdk/src/rp2_common/pico_platform/include/")
        .include("lib/pico-sdk/src/rp2040/hardware_regs/include/")
        .include("lib/pico-sdk/src/rp2040/hardware_structs/include/")
        .compile("can2040");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .ctypes_prefix("cty")
        .use_core()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
