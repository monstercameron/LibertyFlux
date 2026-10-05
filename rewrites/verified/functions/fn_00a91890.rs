// original: 0x00a91890 stream_merge_list_and_children (proposed)

/// Merge a node's list into the accumulator, then merge live children.
///
/// Every value in the `+0x10` list is wrapped onto the accumulator's front
/// (cell from callee 1, thiscall/0 on the allocator from its global; value
/// in front, old head behind; the accumulator is the second argument).
/// Then each nonzero child slot past `+0x14` whose liveness probe (callee 2,
/// thiscall/2 on the object with the first argument and the child index)
/// has a nonzero low byte is merged the same way by recursion (callee 3,
/// thiscall/2 on the child with both arguments).
///
/// Returns the object pointer. Thiscall: object in ecx, the probe key and
/// the accumulator pointer on the stack, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00a91890(this: u32, key: u32, acc: u32) -> u32 {
    unsafe {
        const LIST_OFF: u32 = 0x10;
        const CHILDREN: u32 = 0x14;
        const NCHILD: u32 = 4;
        const ALLOC_GLOBAL: u32 = 0x012b4164;
        let alloc = lf_checker_rt::global::<u32>(ALLOC_GLOBAL).read_unaligned();
        let mut s = ((this + LIST_OFF) as *const u32).read_unaligned();
        while s != 0 {
            let v = (s as *const u32).read_unaligned();
            let cell = lf_checker_rt::callee_thiscall!(1, u32, alloc);
            (cell as *mut u32).write_unaligned(v);
            ((cell + 4) as *mut u32)
                .write_unaligned((acc as *const u32).read_unaligned());
            (acc as *mut u32).write_unaligned(cell);
            s = ((s + 4) as *const u32).read_unaligned();
        }
        for k in 0..NCHILD {
            let child =
                ((this + CHILDREN + k * 4) as *const u32).read_unaligned();
            if child != 0 {
                let live = lf_checker_rt::callee_thiscall!(2, u32, this, key, k);
                if (live & 0xff) != 0 {
                    lf_checker_rt::callee_thiscall!(3, u32, child, key, acc);
                }
            }
        }
        this
    }
});
