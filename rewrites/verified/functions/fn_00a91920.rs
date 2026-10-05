// original: 0x00a91920 stream_graft_list_tree (proposed)

/// Graft a tree of list nodes onto the accumulator's front.
///
/// Starting at the object and descending while the child selector (callee 2,
/// thiscall/1 on the node with the first argument) returns a live child
/// index (not -1 and pointing at a nonzero child slot past `+0x14`), every
/// value in the node's `+0x10` list is wrapped: a cell is allocated
/// (callee 1, thiscall/0 on the allocator from its global), the value goes
/// in front and the accumulator's old head (second argument points at it)
/// behind, and the accumulator takes the cell.
///
/// Returns the last selector answer. Thiscall: object in ecx, the selector
/// key and the accumulator pointer on the stack, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00a91920(this: u32, key: u32, acc: u32) -> u32 {
    unsafe {
        const LIST_OFF: u32 = 0x10;
        const CHILDREN: u32 = 0x14;
        const ALLOC_GLOBAL: u32 = 0x012b4164;
        const NO_CHILD: u32 = 0xffffffff;
        let mut node = this;
        loop {
            let alloc = lf_checker_rt::global::<u32>(ALLOC_GLOBAL).read_unaligned();
            let mut s = ((node + LIST_OFF) as *const u32).read_unaligned();
            while s != 0 {
                let v = (s as *const u32).read_unaligned();
                let cell = lf_checker_rt::callee_thiscall!(1, u32, alloc);
                (cell as *mut u32).write_unaligned(v);
                ((cell + 4) as *mut u32)
                    .write_unaligned((acc as *const u32).read_unaligned());
                (acc as *mut u32).write_unaligned(cell);
                s = ((s + 4) as *const u32).read_unaligned();
            }
            let a2 = lf_checker_rt::callee_thiscall!(2, u32, node, key);
            if a2 == NO_CHILD {
                return a2;
            }
            let child = ((node + CHILDREN + a2.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if child == 0 {
                return a2;
            }
            node = child;
        }
    }
});
