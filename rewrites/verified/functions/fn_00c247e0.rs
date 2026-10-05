// original: 0x00c247e0 cam_interp_rate (proposed)
/// Blend rate from the interpolator's counters: `(base - prev) / span`
/// where `base` is the global tick selected by the flag byte at +0x140
/// (set: 0x011735c4, clear: 0x011735d4), `prev` the word at +0x144 and
/// `span` the word at +0x148. The subtraction wraps; both sides convert
/// from signed 32-bit; the division is one SSE divide.
///
/// Original: 0x00c247e0 (thiscall, no stack words; float result).
lf_checker_rt::export!(thiscall, rw_00c247e0(obj: u32) -> f32 {
    unsafe {
        const TICK_A: u32 = 0x011735d4;
        const TICK_B: u32 = 0x011735c4;
        let flag = (obj.wrapping_add(0x140) as *const u8).read();
        let tick_va = if flag != 0 { TICK_B } else { TICK_A };
        let base = (lf_checker_rt::relocated(tick_va) as *const u32).read_unaligned();
        let prev = (obj.wrapping_add(0x144) as *const u32).read_unaligned();
        let num = base.wrapping_sub(prev) as i32 as f32;
        let span = (obj.wrapping_add(0x148) as *const i32).read_unaligned() as f32;
        core::hint::black_box(num) / core::hint::black_box(span)
    }
});
