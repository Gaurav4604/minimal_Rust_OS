#![no_main] 
// Rust requires C lang runtime to initialize std
// since we are not using std, we overwrite the entry point

#![no_std]
// since we are not using std
// we need to implement our own panic handler


use core::panic::PanicInfo; //this parameter tells at what line the panic occured

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { 
    //this doesn't return any value as it is a diverging function
    loop{}
}

#[no_mangle] 
// since we want to make sure that _start is the first function
pub extern "C" fn _start() -> ! { 
    // we use the no_mangle trait to make sure it is called first (otherwise it would assign a cryptic hash to every function)
    // we use "C" extern as we want to implement C lang calling convention
    loop{}
}

// for building, we need to include the linker file as well ** cargo build --target thumbv7em-none-eabihf **