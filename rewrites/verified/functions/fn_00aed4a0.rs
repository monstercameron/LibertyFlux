// original: 0x00AED4A0 stream_parse_pair (proposed)

/// Parse a key and a float out of two fields into this record.
///
/// The key callee resolves `key_field` and its result is stored at
/// `this+0`; the float callee converts `float_field` (x87 single) and the
/// value lands at `this+4`. Returns the record pointer.
///
/// Original: 0x00AED4A0 (thiscall, two stack words, two direct callees).
lf_checker_rt::export!(thiscall, rw_00aed4a0(this: u32, key_field: u32, float_field: u32) -> u32 {
    unsafe {
        const KEY_CALLEE: u32 = 1;
        const FLOAT_CALLEE: u32 = 2;
        const KEY_OFF: u32 = 0;
        const VAL_OFF: u32 = 4;
        let k: u32 = lf_checker_rt::callee_stdcall!(KEY_CALLEE, u32, key_field);
        ((this.wrapping_add(KEY_OFF)) as *mut u32).write_unaligned(k);
        let f: f32 = lf_checker_rt::callee_cdecl!(FLOAT_CALLEE, f32, float_field);
        ((this.wrapping_add(VAL_OFF)) as *mut f32).write_unaligned(f);
        this
    }
});
