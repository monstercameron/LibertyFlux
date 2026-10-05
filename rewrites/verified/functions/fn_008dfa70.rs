// original: 0x008dfa70 broadcast_table_callback (proposed)

/// Call the virtual hook for every live node of a chained hash table.
///
/// Walks all `count` buckets (16-bit at `this + 0x40`, array at `this +
/// 0x3c`); an empty table returns at once. For each node whose link word at
/// `+4` is non-zero, the hook (virtual slot `+0x20` of the object formed by
/// adding that link to the base at `[this + subindex * 4] + 0x10`, with the
/// sub-index at `this + 0x14`) runs with `arg` through callee 1. Nodes are
/// `[key, link, next]` triples. Thiscall, one stack argument.
lf_checker_rt::export!(thiscall, rw_008dfa70(this: u32, arg: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x3c;
        const COUNT_OFF: u32 = 0x40;
        const SUBINDEX_OFF: u32 = 0x14;
        const HOOK_BASE_OFF: u32 = 0x10;
        const NODE_LINK: u32 = 4;
        const NODE_NEXT: u32 = 8;
        const HOOK_SLOT: u32 = 0x20;
        let count = ((this + COUNT_OFF) as *const u16).read_unaligned() as u32;
        if count == 0 {
            return 0;
        }
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        let subindex = ((this + SUBINDEX_OFF) as *const u32).read_unaligned();
        let base = ((this + subindex.wrapping_mul(4)) as *const u32)
            .read_unaligned()
            .wrapping_add(HOOK_BASE_OFF);
        let mut i: u32 = 0;
        while i < count {
            let mut node =
                ((table + i.wrapping_mul(4)) as *const u32).read_unaligned();
            while node != 0 {
                let link = ((node + NODE_LINK) as *const u32).read_unaligned();
                if link != 0 {
                    let obj = base.wrapping_add(link);
                    let vtable = (obj as *const u32).read_unaligned();
                    let hook = ((vtable + HOOK_SLOT) as *const u32).read_unaligned();
                    let f: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(hook as usize);
                    f(obj, arg);
                }
                node = ((node + NODE_NEXT) as *const u32).read_unaligned();
            }
            i += 1;
        }
        0
    }
});
