#![no_main]
#![no_std]
mod vga_buffer;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  
    vga_buffer::print_something();  
    loop{}
}
