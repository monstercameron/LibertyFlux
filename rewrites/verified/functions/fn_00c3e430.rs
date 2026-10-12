// original: 0x00C3E430 train_xmm_double_call
/// Converts the signed first component and second component to doubles,
/// passes them to the angle helper, and returns the helper's result as a float.
lf_checker_rt::export!(thiscall, rw_00c3e430(this_: u32) -> f32 {
    unsafe {
        const SIGN_MASK_VA: u32 = 0x00FE8FA0;
        let sign_mask = core::ptr::read_unaligned(lf_checker_rt::global::<u32>(SIGN_MASK_VA));
        let first_bits = core::ptr::read_unaligned((this_ + 16) as *const u32) ^ sign_mask;
        let second_bits = core::ptr::read_unaligned((this_ + 20) as *const u32);
        let first = f32::from_bits(first_bits) as f64;
        let second = f32::from_bits(second_bits) as f64;
        let answer_bits = lf_checker_rt::callee_cdecl!(
            1,
            u64,
            answer_word(first.to_bits(), 0),
            answer_word(first.to_bits(), 1),
            answer_word(second.to_bits(), 0),
            answer_word(second.to_bits(), 1)
        );
        f64::from_bits(answer_bits) as f32
    }
});

#[inline(always)]
fn answer_word(bits: u64, high: u32) -> u32 {
    if high == 0 { bits as u32 } else { (bits >> 32) as u32 }
}
