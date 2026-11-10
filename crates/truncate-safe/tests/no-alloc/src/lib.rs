#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn truncated_length(limit: usize) -> usize {
    truncate_safe::truncate("aé中🦀z", limit).len()
}

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
