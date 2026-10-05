// original: 0x00A8A4F0 pool_release_list (proposed)

/// Release every node of the list rooted at `this+0x18`.
///
/// Nodes are entered through their object (`[node]`, called through its
/// function table slot `+0x44`) and linked through `[node + 4]`; the list
/// ends at the embedded anchor `this + 8`, so an empty list makes no
/// calls.
///
/// Original: thiscall, no stack words, no return value. One outgoing
/// call shape (thiscall through the object's table, no stack words),
/// intercepted by a planted stub address.
lf_checker_rt::export!(thiscall, rw_00A8A4F0(this: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x18;
        const ANCHOR_OFF: u32 = 8;
        const NEXT_OFF: u32 = 4;
        const RELEASE_SLOT: u32 = 0x44;
        let end = this.wrapping_add(ANCHOR_OFF);
        let mut node = ((this + HEAD_OFF) as *const u32).read_unaligned();
        while node != end {
            let obj = (node as *const u32).read_unaligned();
            node = ((node + NEXT_OFF) as *const u32).read_unaligned();
            let slot = ((((obj as *const u32).read_unaligned()) + RELEASE_SLOT)
                as *const u32)
                .read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            let _ = f(obj);
        }
        0
    }
});
