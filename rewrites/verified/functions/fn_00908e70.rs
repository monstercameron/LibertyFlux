// original: 0x00908e70 input_store_two_floats (proposed)
/// Store two incoming float arguments into two static float slots.
///
/// `a` is stored to the first slot and `b` to the second; both are plain
/// 32-bit copies (SSE `movss`), so every bit pattern including NaNs is
/// preserved. The function returns nothing meaningful (cdecl, two stack
/// words, no return value in `eax`).
export!(cdecl, rw_00908e70(a: u32, b: u32) -> u32 {
    unsafe {
        /// First static float slot (file VA).
        const SLOT_A: u32 = 0x01190E6C;
        /// Second static float slot (file VA).
        const SLOT_B: u32 = 0x01193C58;
        (global::<u32>(SLOT_A)).write_unaligned(a);
        (global::<u32>(SLOT_B)).write_unaligned(b);
        0
    }
});
