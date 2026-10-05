// original: 0x00a91990 stream_absorb_list_checked (proposed)

/// Absorb a node's list values the visitor rejects, then absorb children.
///
/// Each value in the `+0x10` list the visitor (callee 1, thiscall/1 on the
/// accumulator with the value) rejects with a zero low byte is wrapped: a
/// cell is allocated (callee 2, thiscall/0 on the allocator from its
/// global), the value goes in front and the accumulator's old head behind,
/// and the accumulator (the argument points at it) takes the cell. Then
/// each nonzero child slot past `+0x14` is absorbed the same way by
/// recursion (callee 3, thiscall/1 on the child with the accumulator).
///
/// Returns the last callee answer, or 0 when nothing ran (entry eax is
/// pinned to 0 by the contract). Thiscall: object in ecx, the accumulator
/// pointer on the stack, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a91990(this: u32, acc: u32) -> u32 {
    unsafe {
        const LIST_OFF: u32 = 0x10;
        const CHILDREN: u32 = 0x14;
        const NCHILD: u32 = 4;
        const ALLOC_GLOBAL: u32 = 0x012b4164;
        let mut ret = 0u32;
        let mut s = ((this + LIST_OFF) as *const u32).read_unaligned();
        while s != 0 {
            let v = (s as *const u32).read_unaligned();
            let a1 = lf_checker_rt::callee_thiscall!(1, u32, acc, v);
            ret = a1;
            if (a1 & 0xff) == 0 {
                let alloc =
                    lf_checker_rt::global::<u32>(ALLOC_GLOBAL).read_unaligned();
                let cell = lf_checker_rt::callee_thiscall!(2, u32, alloc);
                ret = cell;
                (cell as *mut u32).write_unaligned(v);
                ((cell + 4) as *mut u32)
                    .write_unaligned((acc as *const u32).read_unaligned());
                (acc as *mut u32).write_unaligned(cell);
            }
            s = ((s + 4) as *const u32).read_unaligned();
        }
        for k in 0..NCHILD {
            let child =
                ((this + CHILDREN + k * 4) as *const u32).read_unaligned();
            if child != 0 {
                ret = lf_checker_rt::callee_thiscall!(3, u32, child, acc);
            }
        }
        ret
    }
});
