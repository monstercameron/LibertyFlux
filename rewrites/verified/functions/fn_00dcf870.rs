// original: 0x00dcf870 ui_node_clear_children (proposed)

/// Destroy every child of the node, release the child array block, and
/// clear the array pointer at `+0x4`. A null array pointer does nothing.
///
/// `this` points to the node. The array descriptor holds the entry base at
/// `+0x0` and a 16-bit count at `+0x4`. Each nonzero entry is passed in ECX
/// to the destructor callee (thiscall, no stack words) and then on the
/// stack to the deallocator callee (cdecl, one word); the bounds are
/// re-read from the descriptor every iteration. Afterwards the release
/// callee (stdcall: base, end) frees the block and the pointer is cleared.
/// No result.
///
/// Original: 0x00DCF870 (thiscall, no stack arguments, no result).
lf_checker_rt::export!(thiscall, rw_00dcf870(this: u32) -> u32 {
    unsafe {
        /// Child destructor callee id.
        const DESTROY: u32 = 1;
        /// Child deallocator callee id.
        const FREE: u32 = 2;
        /// Array-block release callee id.
        const RELEASE: u32 = 3;
        const KIDS: u32 = 4;
        let desc = ((this + KIDS) as *const u32).read_unaligned();
        if desc != 0 {
            let base = (desc as *const u32).read_unaligned();
            let count = ((desc + 4) as *const u16).read_unaligned() as u32;
            let mut p = base;
            let end = base.wrapping_add(count.wrapping_mul(4));
            while p != end {
                let entry = (p as *const u32).read_unaligned();
                if entry != 0 {
                    lf_checker_rt::callee_thiscall!(DESTROY, u32, entry);
                    lf_checker_rt::callee_cdecl!(FREE, u32, entry);
                }
                // Bounds re-read every iteration, as in the original.
                let base = (desc as *const u32).read_unaligned();
                let count = ((desc + 4) as *const u16).read_unaligned() as u32;
                p = p.wrapping_add(4);
                if p == base.wrapping_add(count.wrapping_mul(4)) {
                    break;
                }
            }
            let base = (desc as *const u32).read_unaligned();
            let count = ((desc + 4) as *const u16).read_unaligned() as u32;
            lf_checker_rt::callee_stdcall!(RELEASE, u32, base, base.wrapping_add(count.wrapping_mul(4)));
            ((this + KIDS) as *mut u32).write_unaligned(0);
        }
        0
    }
});
