// original: 0x009673E0 attach_free_node
/// Pop a node off the global free list and push it on `this + 0x30`.
///
/// Under the guard (callee 1 locks, callee 2 unlocks) the head of the
/// global free list at file VA `0x01218474` is removed; an empty free
/// list ends the function with no effect. Otherwise the node is linked
/// at the head of the list at `this + 0x30` and its key word set to
/// `key`. No return value.
///
/// Original: 0x009673E0 (thiscall, one stack word).

export!(thiscall, rw_009673E0(this: u32, key: u32) -> u32 {
    unsafe {
        const FREE_HEAD: u32 = 0x01218474;
        const LIST: u32 = 0x30;
        callee_cdecl!(1, u32,);
        let head = *global::<u32>(FREE_HEAD);
        if head != 0 {
            let next = (head as *const u32).read_unaligned();
            *global::<u32>(FREE_HEAD) = next;
            let old = (this.wrapping_add(LIST) as *const u32).read_unaligned();
            (head as *mut u32).write_unaligned(old);
            (this.wrapping_add(LIST) as *mut u32).write_unaligned(head);
            (head.wrapping_add(4) as *mut u32).write_unaligned(key);
        }
        callee_cdecl!(2, u32,);
        0
    }
});
