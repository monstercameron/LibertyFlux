// original: 0x00a7ca00 sweep_touch_and_link
// Sweep the +0x120 chain from the callee-provided head: for every
// node flagged with bit 2 (+0x13c & 4), touch it when unclaimed
// ([+0x130] == 0) and adopt the first one with bit 3 set and bit 1
// clear as the anchor; nodes without bit 2 are skipped outright. Then
// link the anchor through the link step and clear the anchor block's
// +0x78 slot. Returns 1 when anything was touched, else 0.
///
/// Proven scope: one head node has flags 0x0c, an unclaimed slot, a null
/// next link, and a fixed sibling pointer. Head, touch, and link are stubs
/// with fixed answers. Other list lengths, flag combinations, claimed
/// states, and null paths are untested.
export!(thiscall, rw_s13_00a7ca00(this: u32) -> u8 {
    unsafe {
        let head: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let touch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let link: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        let mut node = head(this);
        if node == 0 {
            return 0;
        }
        let mut touched_any = false;
        let mut anchor = 0u32;
        while node != 0 {
            let flags = *((node + 0x13C) as *const u8);
            if flags & 0x04 != 0 {
                if *((node + 0x130) as *const u32) == 0 {
                    touch(node);
                    touched_any = true;
                }
                if anchor == 0 && flags & 0x08 != 0 && flags & 0x02 == 0 {
                    anchor = node;
                }
            }
            node = *((node + 0x120) as *const u32);
        }
        if anchor != 0 {
            let sibling = *((anchor + 0x118) as *const u32);
            link(sibling.wrapping_add(0x10), anchor.wrapping_add(0x10));
            *((sibling + 0x78) as *mut u32) = 0;
        }
        touched_any as u8
    }
});
