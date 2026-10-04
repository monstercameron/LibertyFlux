// original: 0x00c5c560 task_timer_fire_and_chain (proposed)

/// Fire a task timer entry and chain to its follow-up.
///
/// Marks the timer at `+8`, looks the entry up in the global table by the
/// index at `+4`, invokes callee 1 on the entry, then re-reads the entry and
/// returns whatever callee 2 (reached by a tail jump in the original) answers
/// for it. Both callees take the entry address plus 8 in ecx.
///
/// Original: 0x00c5c560 (thiscall: `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_00c5c560(this: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x12b60a0;
        const INDEX: u32 = 4;
        const FIRED: u32 = 8;
        const FIRE: u32 = 1;
        const CHAIN: u32 = 2;
        let index = ((this + INDEX) as *const u32).read_unaligned();
        ((this + FIRED) as *mut u8).write(1);
        let base = lf_checker_rt::relocated(TABLE);
        let entry = ((base.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(FIRE, u32, entry.wrapping_add(8));
        let entry2 = ((base.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(CHAIN, u32, entry2.wrapping_add(8))
    }
});

