// original: 0x00d8ed10 audio_float_lookup_mark
/// Look up the handle for `key` at level `level_bits` and mark it active.
///
/// Forwards the key, the level bits and a result slot to the shared lookup.
/// When the lookup reports success through its low byte and returns a
/// nonzero handle, bit 0x10000000 is set in the handle's status word.
/// Returns the lookup's answer byte-extended state as the full answer word.
lf_rs89_rt::export!(thiscall, rw_00d8ed10(this: u32, key: u32, level_bits: u32) -> u32 {
    unsafe {
        let mut handle: u32 = 0;
        let ans: u32 = lf_rs89_rt::callee_thiscall!(
            1,
            u32,
            this,
            key,
            level_bits,
            &mut handle as *mut u32 as u32
        );
        if ans & 0xFF != 0 && handle != 0 {
            let status = handle.wrapping_add(4) as *mut u32;
            *status |= 0x10000000;
        }
        ans
    }
});
