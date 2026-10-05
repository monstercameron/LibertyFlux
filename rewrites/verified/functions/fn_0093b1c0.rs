// original: 0x0093B1C0 stream_stdcall_wrap_b (proposed)

/// Store a value into this record's slot, unless it is null.
///
/// `this` points to the record. A null `value` answers 0 without calling.
/// Otherwise calls the setter with destination `this+0x10F`, full mask
/// `0xFF` and the value, returning the setter's answer (thiscall).
lf_checker_rt::export!(thiscall, rw_0093b1c0(this: u32, value: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x10F;
        const FULL_MASK: u32 = 0xFF;
        const SETTER: u32 = 1;
        if value == 0 {
            0
        } else {
            lf_checker_rt::callee_cdecl!(SETTER, u32, this + SLOT, FULL_MASK, value)
        }
    }
});
