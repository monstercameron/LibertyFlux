// original: 0x00AEFB20 stream_hashed_float (proposed)

/// Hash a name with a zero seed and answer the table's float for it.
///
/// `name` is hashed with seed 0, then the named-float callee is asked with
/// (this, hash, `hint`) where `hint` is the second stack word (passed
/// through this call's own scratch slot). The callee's x87 float result is
/// returned unchanged.
///
/// Original: 0x00AEFB20 (thiscall, two stack words, two direct callees).
lf_checker_rt::export!(thiscall, rw_00aefb20(this: u32, name: u32, hint: u32) -> f32 {
    unsafe {
        const HASH_CALLEE: u32 = 1;
        const TABLE_CALLEE: u32 = 2;
        let h: u32 = lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, name, 0);
        lf_checker_rt::callee_thiscall!(TABLE_CALLEE, f32, this, h, hint)
    }
});
