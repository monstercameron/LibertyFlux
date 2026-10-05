// original: 0x00878490 conduit_entry_unlink

/// Unlink a conduit entry's nodes up to its live target.
///
/// `this` points to a conduit object. The helper's answer is subtracted
/// from the base (`+0x10`) plus `a0` to form an index; when the slot at
/// `+0x18 + index * 4` is null there is nothing to do. A negative index
/// (SIGNED comparison) takes the head entry (`[this]`); otherwise slots
/// at `+0x14` are scanned down from the index while null (below zero
/// takes the head entry too) and the entry is the found slot's content
/// plus 4. The target is the top slot's word at `+4`: when the entry's
/// first word already equals it only the slot is cleared, otherwise
/// nodes move one by one from the entry list to the holder list at
/// `[this+4]` until the entry's front is the target, then the slot is
/// cleared. No return value.
///
/// Original: 0x00878490 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00878490(this: u32, a0: u32) -> u32 {
    unsafe {
        const BASE_OFF: u32 = 0x10;
        const TOP_OFF: u32 = 0x18;
        const SCAN_OFF: u32 = 0x14;
        const NEXT_OFF: u32 = 0x04;
        const HOLDER_OFF: u32 = 0x04;
        const HELPER: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let answer = lf_checker_rt::callee_thiscall!(HELPER, u32, this);
        let base = rd32(this.wrapping_add(BASE_OFF));
        let mut index =
            base.wrapping_add(a0).wrapping_sub(answer) as i32;
        let slot = this
            .wrapping_add(TOP_OFF)
            .wrapping_add((index as u32).wrapping_mul(4));
        let top = rd32(slot);
        if top == 0 {
            return 0;
        }
        let entry: u32;
        if index < 0 {
            entry = rd32(this);
        } else {
            let mut probe = this
                .wrapping_add(SCAN_OFF)
                .wrapping_add((index as u32).wrapping_mul(4));
            loop {
                let found = rd32(probe);
                if found != 0 {
                    entry = found.wrapping_add(NEXT_OFF);
                    break;
                }
                probe = probe.wrapping_sub(4);
                index -= 1;
                if index < 0 {
                    entry = rd32(this);
                    break;
                }
            }
        }
        let target = rd32(top.wrapping_add(NEXT_OFF));
        if rd32(entry) != target {
            loop {
                let node = rd32(entry);
                let next = rd32(node.wrapping_add(NEXT_OFF));
                let holder = rd32(this.wrapping_add(HOLDER_OFF));
                wr32(node.wrapping_add(NEXT_OFF), rd32(holder));
                wr32(holder, node);
                wr32(entry, next);
                if next == target {
                    break;
                }
            }
        }
        wr32(slot, 0);
        0
    }
});
