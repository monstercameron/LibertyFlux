// original: 0x00a71d40 NativeImpl_IS_PLAYER_FREE_AIMING_AT_CHAR_2 (symbols)
/// If `out` is non-null, stores the object's field at `+0x38` there, then
/// returns the field at `+0x34` when the flag byte (low byte of `flag`) is
/// non-zero, else the field at `+0x30`.
///
/// `thiscall`: object in ECX, two stack words, callee pops 8. Leaf: reads
/// three words off `this`, writes one word through `out` at most. Edge
/// cases: null `out` skips the store; only the flag's low byte matters.
lf_checker_rt::export!(thiscall, rw_00a71d40(this: u32, out: u32, flag: u32) -> u32 {
    unsafe {
        const FIELD_ALT: u32 = 0x30;
        const FIELD_MAIN: u32 = 0x34;
        const FIELD_SAVED: u32 = 0x38;
        if out != 0 {
            ((out) as *mut u32)
                .write_unaligned(((this + FIELD_SAVED) as *const u32).read_unaligned());
        }
        if (flag as u8) != 0 {
            ((this + FIELD_MAIN) as *const u32).read_unaligned()
        } else {
            ((this + FIELD_ALT) as *const u32).read_unaligned()
        }
    }
});
