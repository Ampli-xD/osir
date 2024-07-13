#![no_std] //does not links the standard libraries of rust
#![no_main]//disables all rust level entry points
mod vga_buffer;
use core::panic::PanicInfo;
use core::fmt::Write;

//This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]//dont mangle the name of this function 
pub extern "C" fn _start() -> ! {
    //this function is the new entry point as most linkers
    //look for _start function

    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop{}
}
