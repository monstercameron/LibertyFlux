// original: 0x00878440 conduit_entry_find

/// Find the live conduit entry at or below a computed index.
///
/// `this` points to a conduit object. The helper's answer is subtracted
/// from the base (`+0x10`) plus `a0` to form an index; when the slot at
/// `+0x18 + index * 4` is null there is no entry and 0 is returned. A
/// negative index (SIGNED comparison) resolves through the head pointer
/// (`[[[this]]]`); otherwise slots at `+0x14` are scanned down from the
/// index while null, resolving below zero the same way, and the live
/// entry's word at `+4` is returned.
///
/// Original: 0x00878440 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00878440(this: u32, a0: u32) -> u32 {
    unsafe {
        const BASE_OFF: u32 = 0x10;
        const TOP_OFF: u32 = 0x18;
        const SCAN_OFF: u32 = 0x14;
        const ENTRY_OFF: u32 = 0x04;
        const HELPER: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let answer = lf_checker_rt::callee_thiscall!(HELPER, u32, this);
        let base = rd32(this.wrapping_add(BASE_OFF));
        let mut index =
            base.wrapping_add(a0).wrapping_sub(answer) as i32;
        let top = rd32(
            this.wrapping_add(TOP_OFF).wrapping_add((index as u32).wrapping_mul(4)));
        if top == 0 {
            return 0;
        }
        if index < 0 {
            let head = rd32(this);
            let first = rd32(head);
            return rd32(first);
        }
        let mut slot = this
            .wrapping_add(SCAN_OFF)
            .wrapping_add((index as u32).wrapping_mul(4));
        while rd32(slot) == 0 {
            slot = slot.wrapping_sub(4);
            index -= 1;
            if index < 0 {
                let head = rd32(this);
                let first = rd32(head);
                return rd32(first);
            }
        }
        rd32(rd32(slot).wrapping_add(ENTRY_OFF))
    }
});
