// original: 0x00AEFAD0 stream_named_float (proposed)

/// Look up a hashed name in the stream table, answer a float.
///
/// `key` is re-hashed with the table's seed string; the table callee is
/// asked with (this, rehash, out-slot) where out-slot addresses this
/// call's own second stack word. When the callee reports a hit (nonzero
/// low byte) that slot's float is the answer; otherwise the callee is
/// asked again with the unhashed `key` and the slot's float is the answer
/// regardless. Single precision; the float result travels on the x87 stack.
///
/// Original: 0x00AEFAD0 (thiscall, two stack words, two direct callees;
// the table callee takes a frame-pointer out argument).
lf_checker_rt::export!(thiscall, rw_00aefad0(this: u32, key: u32, _f: u32) -> f32 {
    unsafe {
        const HASH_CALLEE: u32 = 1;
        const TABLE_CALLEE: u32 = 2;
        const SEED_STR: u32 = 0x00EA7D90;
        let rehash: u32 = lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, lf_checker_rt::relocated(SEED_STR), key);
        let mut out: u32 = _f;
        let hit: u32 = lf_checker_rt::callee_thiscall!(TABLE_CALLEE, u32, this, rehash, core::ptr::addr_of_mut!(out) as u32);
        if (hit as u8) == 0 {
            lf_checker_rt::callee_thiscall!(TABLE_CALLEE, u32, this, key, core::ptr::addr_of_mut!(out) as u32);
        }
        f32::from_bits(out)
    }
});
