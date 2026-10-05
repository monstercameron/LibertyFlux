// original: 0x00a8a200 pool_foreach_accumulate (proposed)

/// Walk the laman chain, accumulating one value per live node.
///
/// `this` is the pool object. Repeatedly asks the head callee for the next
/// node until it answers null (the proof scripts two nodes then null).
/// Each node whose word at +0 is non-zero is offered to the prepare callee
/// with node+0x10, then to the value callee with (node, [handle+0xC],
/// chain-link), and the answers are summed. Returns the sum.
///
/// Original: 0x00A8A200 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a8a200(this: u32) -> u32 {
    unsafe {
        const CALLEE_NEXT: u32 = 1;
        const CALLEE_PREPARE: u32 = 2;
        const CALLEE_VALUE: u32 = 3;
        const NODE_HEAD: u32 = 0;
        const NODE_HANDLE: u32 = 4;
        const PREPARE_ARG: u32 = 0x10;
        const VALUE_SLOT: u32 = 0xc;
        let mut total = 0u32;
        let mut link =
            lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, this);
        while link != 0 {
            let node =
                ((link + NODE_HEAD) as *const u32).read_unaligned();
            if node != 0 {
                lf_checker_rt::callee_thiscall!(
                    CALLEE_PREPARE,
                    u32,
                    this,
                    node.wrapping_add(PREPARE_ARG)
                );
                let handle =
                    ((node + NODE_HANDLE) as *const u32).read_unaligned();
                let slot =
                    ((handle + VALUE_SLOT) as *const u32).read_unaligned();
                let v = lf_checker_rt::callee_thiscall!(
                    CALLEE_VALUE,
                    u32,
                    this,
                    link,
                    slot,
                    node
                );
                total = total.wrapping_add(v);
            }
            link = lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, this);
        }
        total
    }
});
