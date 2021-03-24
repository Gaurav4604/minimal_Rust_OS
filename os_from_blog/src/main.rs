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


static HELLO: &[u8] = b"Hello World This is Gaurav Singh"; //ascii value buffer for ** hello world **

#[no_mangle] 
// since we want to make sure that _start is the first function
pub extern "C" fn _start() -> ! { 
    // we use the no_mangle trait to make sure it is called first (otherwise it would assign a cryptic hash to every function)
    // we use "C" extern as we want to implement C lang calling convention
    // linker will use this _start as the entry point of program

    let vga_buffer = 0xb8000 as *mut u8; // 0xb8000 is casted into a raw pointer
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize*2) = byte;
            *vga_buffer.offset(i as isize*2 + 1) = 0xa;
        }
    }
    loop{}
}

// for building, we need to include the linker file as well ** cargo build --target thumbv7em-none-eabihf **

// for booting the os ** qemu-system-x86_64 -drive format=raw,file=target/x86_64_os_setup_file/debug/bootimage-os_from_blog.bin -L "D:\Softwares\emulators\qemu" **