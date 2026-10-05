// original: 0x00a91120 stream_lookup_slot_checked (proposed)

/// Resolve the argument through a three-out-slot lookup, then zero a word.
///
/// Three stack out-slots (each pre-set to 0x3f) and the argument go to the
/// resolver (callee 1, thiscall/4). When the first out-slot still holds 0x3f
/// the resolver's answer is returned; when it holds the slot limit at
/// `this+0xe8` or more (signed) the limit is returned. Otherwise the slot at
/// index `out*0xa0` past `this+0xe4` is opened (callee 2, thiscall/1 with
/// 0x10) and its first word is zeroed.
///
/// Returns the resolver answer, the limit, or the opened pointer.
/// Thiscall: object in ecx, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a91120(this: u32, arg: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0xe4;
        const LIMIT: u32 = 0xe8;
        const SLOT_STRIDE: u32 = 0xa0;
        const UNSET: u32 = 0x3f;
        const OPEN_ARG: u32 = 0x10;
        const RESOLVE_CALLEE: u32 = 1;
        const OPEN_CALLEE: u32 = 2;
        let mut s1 = UNSET;
        let mut s2 = UNSET;
        let mut s3 = UNSET;
        let ans = lf_checker_rt::callee_thiscall!(
            RESOLVE_CALLEE,
            u32,
            this,
            arg,
            &mut s1 as *mut u32 as u32,
            &mut s2 as *mut u32 as u32,
            &mut s3 as *mut u32 as u32
        );
        if s1 == UNSET {
            return ans;
        }
        let limit = ((this + LIMIT) as *const u16).read_unaligned() as u32;
        if (s1 as i32) >= (limit as i32) {
            return limit;
        }
        let slot = ((this + SLOTS) as *const u32).read_unaligned()
            + s1.wrapping_mul(5).wrapping_mul(32);
        let p = lf_checker_rt::callee_thiscall!(OPEN_CALLEE, u32, slot, OPEN_ARG);
        (p as *mut u32).write_unaligned(0);
        p
    }
});
