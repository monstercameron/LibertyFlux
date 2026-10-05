// original: 0x00CC6000 list_delete_all (proposed)

/// Pop every node off the queue at `this` and free each one.
///
/// Repeatedly pops the head node (through the pop callee) and hands it to the
/// freeing callee until the head word reads null. An already-empty queue
/// makes no calls. Returns nothing meaningful: the original falls through
/// with whatever the last call left in the return register.
///
/// Original: 0x00CC6000 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00cc6000(this: u32) -> u32 {
    unsafe {
        const POP_CALLEE: u32 = 1;
        const FREE_CALLEE: u32 = 2;
        const HEAD: u32 = 0;
        loop {
            if (this.wrapping_add(HEAD) as *const u32).read_unaligned() == 0 {
                break;
            }
            let node = lf_checker_rt::callee_thiscall!(POP_CALLEE, u32, this);
            lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, node);
        }
        0
    }
});
