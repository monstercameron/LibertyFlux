// original: 0x0086F1D0 SIN
//! Converts the input angle with the image's degrees-to-radians factor,
//! calls the engine sine routine, and stores the returned float bits.

use core::hint::black_box;

const DEGREES_TO_RADIANS_VA: u32 = 0x00FE8728;

lf_checker_rt::export!(cdecl, rw_fn_0086f1d0(context: u32) -> () {
    let context_words = context as *const u32;
    let output = unsafe { core::ptr::read_unaligned(context_words) };
    let inputs = unsafe { core::ptr::read_unaligned(context_words.add(2)) };
    let input_bits = unsafe { core::ptr::read_unaligned(inputs as *const u32) };
    let angle = f32::from_bits(input_bits);
    let factor = unsafe { lf_checker_rt::global::<f32>(DEGREES_TO_RADIANS_VA).read() };
    let radians = black_box(angle) * black_box(factor);
    let result = lf_checker_rt::callee_cdecl!(1, u32, radians.to_bits());
    unsafe { core::ptr::write_unaligned(output as *mut u32, result) };
});
