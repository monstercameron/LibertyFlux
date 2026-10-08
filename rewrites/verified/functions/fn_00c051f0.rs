#![no_std]

use lf_checker_rt::{callee_thiscall, export};

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        core::ptr::write_volatile(core::ptr::null_mut::<u8>(), 0);
        core::hint::unreachable_unchecked()
    }
}

/// Creates two helper-output records and forwards them with the caller inputs.
/// The proof observes four initialized words in the first record and three in
/// the second. Both helper bodies are scripted; raw local pointer addresses
/// are skipped while their initialized contents are compared. The separate
/// caller-vector pointee and native helper computations remain outside scope.
/// Full data-section comparison is disabled.
unsafe fn run_wrapper(this: u32, first: u32, second: u32, third: u32, weight_bits: u32) -> u32 {
    let mut scratch = [0u32; 10];
    let base = scratch.as_mut_ptr() as u32;
    let first_record = base.wrapping_add(16);
    let second_record = base;

    let _ = callee_thiscall!(1, u32, this, first_record, second_record);
    callee_thiscall!(2, u32, this, first, first_record, second_record, second, third, weight_bits)
}

export!(thiscall, rw_00c051f0(this: u32, first: u32, second: u32, third: u32, weight_bits: u32) -> u32 {
    unsafe { run_wrapper(this, first, second, third, weight_bits) }
});
