#![no_std]
#![no_main]
use cortex_m_rt::entry;
const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
#[entry]
fn main() -> ! {
unsafe {
core::ptr::write_volatile(DDRB, 0b00100000);
loop{
for _ in 0..1_000_000 {
core::ptr::write_volatile(PORTB, 0b00100000);
}
for _ in 0..100_000 {
core::ptr::write_volatile(PORTB, 0b00000000);
}
}
}
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
loop {}
}