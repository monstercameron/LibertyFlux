// original: 0x00be2bf0 task_dispatch_indexed_child_hi (proposed)

/// Dispatch to an indexed child task through its second vtable slot.
///
/// `holder` (second stack word; the first and third are unread) points at a
/// word holding the child index. Reads the index, loads the child pointer at
/// `this + TABLE_OFF + index * 4` (0x24), and returns zero when the slot is
/// empty. Otherwise calls the child's second virtual (`*(child + 0)` is its
/// vtable, slot `VF1_SLOT` is 0x4) with the child as object, and returns
/// whatever that call answers. The index is trusted: no bounds check.
///
/// Original: 0x00be2bf0 (thiscall, three stack words: unread, holder, unread).
lf_checker_rt::export!(thiscall, rw_00be2bf0(this: u32, _a0: u32, holder: u32, _a2: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x24;
        const VF1_SLOT: u32 = 0x04;
        let index = (holder as *const u32).read_unaligned();
        let child = (this
            .wrapping_add(TABLE_OFF)
            .wrapping_add(index.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        if child == 0 {
            return 0;
        }
        let vtable = (child as *const u32).read_unaligned();
        let target = (vtable.wrapping_add(VF1_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        f(child)
    }
});
