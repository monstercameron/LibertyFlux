// original: 0x008785B0 conduit_compact

/// Compact a conduit: unlink every slot but one, then repack the top.
///
/// `this` points to a conduit object. The helper counts the slots; every
/// slot except `a0` is unlinked through the unlink helper (SIGNED loop:
/// a count of zero or less, and an index reaching the count, end it).
/// Then the base (`+0x10`) is scanned down from while its slots at
/// `+0x14` are null (a negative base, SIGNED, skips the scan): the
/// found slot's content plus 4, or the head (`[this]`), is the drain.
/// The top slot at `+0x18 + (base - count + a0) * 4` moves to the scan
/// slot at `+0x14 + (base - count + 1) * 4`, the base becomes
/// `base - count + 1`, the pending node at `[this+8]` has its next
/// word stored to the drain and itself moves to the front of the
/// holder list at `[this+4]`, and `[this+8]` is cleared. No return.
///
/// Original: 0x008785B0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_008785B0(this: u32, a0: u32) -> u32 {
    unsafe {
        const BASE_OFF: u32 = 0x10;
        const TOP_OFF: u32 = 0x18;
        const SCAN_OFF: u32 = 0x14;
        const NEXT_OFF: u32 = 0x04;
        const HOLDER_OFF: u32 = 0x04;
        const PENDING_OFF: u32 = 0x08;
        const COUNT_HELPER: u32 = 1;
        const UNLINK_HELPER: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let count =
            lf_checker_rt::callee_thiscall!(COUNT_HELPER, u32, this) as i32;
        if count > 0 {
            let mut i = 0i32;
            while i < count {
                if i != a0 as i32 {
                    let _kept: u32 = lf_checker_rt::callee_thiscall!(
                        UNLINK_HELPER, u32, this, i as u32);
                }
                i += 1;
            }
        }
        let base = rd32(this.wrapping_add(BASE_OFF)) as i32;
        let drain: u32;
        if base < 0 {
            drain = rd32(this);
        } else {
            let mut probe = this
                .wrapping_add(SCAN_OFF)
                .wrapping_add((base as u32).wrapping_mul(4));
            let mut k = base;
            loop {
                let found = rd32(probe);
                if found != 0 {
                    drain = found.wrapping_add(NEXT_OFF);
                    break;
                }
                probe = probe.wrapping_sub(4);
                k -= 1;
                if k < 0 {
                    drain = rd32(this);
                    break;
                }
            }
        }
        let rest = base.wrapping_sub(count);
        let top = rd32(this.wrapping_add(TOP_OFF).wrapping_add(
            (rest.wrapping_add(a0 as i32) as u32).wrapping_mul(4)));
        let moved = rest.wrapping_add(1);
        wr32(this.wrapping_add(BASE_OFF), moved as u32);
        wr32(this
            .wrapping_add(SCAN_OFF)
            .wrapping_add((moved as u32).wrapping_mul(4)), top);
        let pending = rd32(this.wrapping_add(PENDING_OFF));
        wr32(drain, rd32(pending.wrapping_add(NEXT_OFF)));
        let holder = rd32(this.wrapping_add(HOLDER_OFF));
        wr32(pending.wrapping_add(NEXT_OFF), rd32(holder));
        wr32(holder, pending);
        wr32(this.wrapping_add(PENDING_OFF), 0);
        0
    }
});
