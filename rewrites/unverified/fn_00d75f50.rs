// original: 0x00d75f50 accumulate_float_pair_forward_flag0
/// Accumulate a float pair with the incoming levels and forward them.
///
/// Runs the setup helper twice with fixed codes, adds the two incoming
/// levels onto the float pair at the incoming pointer, and when this
/// object's flag word is set, resets the object through the second helper
/// and forwards the four words (pair plus sums) with the slot word to the
/// sink helper, first marking the slot -1. Returns the sink's answer, or
/// the setup answer when the flag word is clear.
export!(thiscall, rw_00d75f50(this_: *const u8, pair: *const f32, first: f32, second: f32) -> u32 {
    unsafe {
        use core::hint::black_box;
        callee_cdecl!(1, u32, 0xa, 0);
        let setup_answer = callee_cdecl!(1, u32, 7, 1);
        let f0 = *pair;
        let f1 = *pair.add(1);
        let sum0 = black_box(black_box(f0) + black_box(first));
        let sum1 = black_box(black_box(f1) + black_box(second));
        if *(this_ as *const u32) == 0 {
            return setup_answer;
        }
        callee_thiscall!(2, u32, this_ as u32);
        let words = [f0.to_bits(), f1.to_bits(), sum0.to_bits(), sum1.to_bits()];
        let mut slot: u32 = 0xffffffff;
        callee_cdecl!(
            3,
            u32,
            words.as_ptr() as u32,
            &mut slot as *mut u32 as u32
        )
    }
});
