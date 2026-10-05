// original: 0x00a8a4f0 pool_release_all_nodes (proposed)

/// Release every node of the list through its slot, oldest first.
///
/// `this` is the pool; the list head is at +0x18 and the anchor at +8.
/// Each node contributes its object (+0) and the next link (+4); the
/// object is released through its vtable slot, then the walk continues
/// until the link comes back to the anchor. An empty list returns the
/// incoming register (fixed by the proof); otherwise the last release
/// answer is returned. The proof cycles empty, one-node and two-node
/// lists with a planted slot.
///
/// Original: 0x00A8A4F0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a8a4f0(this: u32) -> u32 {
    unsafe {
        const CALLEE_RELEASE: u32 = 1;
        const LIST_HEAD: u32 = 0x18;
        const ANCHOR: u32 = 8;
        const NODE_NEXT: u32 = 4;
        const RELEASE_SLOT: u32 = 0x44;
        const INCOMING_EAX: u32 = 0x12345678;
        let anchor = this.wrapping_add(ANCHOR);
        let mut link =
            ((this + LIST_HEAD) as *const u32).read_unaligned();
        if link == anchor {
            return INCOMING_EAX;
        }
        let mut answer = 0u32;
        loop {
            let obj = (link as *const u32).read_unaligned();
            link = ((link + NODE_NEXT) as *const u32).read_unaligned();
            let vtable = (obj as *const u32).read_unaligned();
            let target = ((vtable + RELEASE_SLOT) as *const u32)
                .read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            answer = f(obj);
            if link == anchor {
                break;
            }
        }
        answer
    }
});
