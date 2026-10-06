// original: 0x0099DE30 audio_key_positive_check (proposed)

/// Report whether the keyed audio query for `arg` returns positive.
///
/// The key comes from `this`+0x9C, or when that is zero from the id helper
/// (callee 1, thiscall/1 of constant 1) followed by the manager lookup
/// (callee 2, thiscall/1 on the shared audio manager). The predicate
/// (callee 3, cdecl/2 of the key and `arg`) answers a SIGNED integer; the
/// result is 1 when it is above zero and 0 otherwise. Only `al` is set, so
/// the upper 24 bits of the return keep the predicate's answer.
lf_checker_rt::export!(thiscall, rw_0099DE30(this: u32, arg: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x9C;
        const MANAGER: u32 = 0x01288780;
        const ID_CALLEE: u32 = 1;
        const LOOKUP_CALLEE: u32 = 2;
        const PRED_CALLEE: u32 = 3;
        let mut key = ((this.wrapping_add(KEY)) as *const u32).read_unaligned();
        if key == 0 {
            let id = lf_checker_rt::callee_thiscall!(ID_CALLEE, u32, this, 1);
            key = lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32,
                                                  lf_checker_rt::relocated(MANAGER), id);
        }
        let ans = lf_checker_rt::callee_cdecl!(PRED_CALLEE, u32, key, arg);
        // Signed greater-than-zero, as the original's `setg`.
        let gt = if (ans as i32) > 0 { 1u32 } else { 0u32 };
        (ans & 0xFFFF_FF00) | gt
    }
});
