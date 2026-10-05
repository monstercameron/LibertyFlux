// original: 0x00a9d940 stream_sync_nodes (proposed)

/// Push the sync value through every node, then run the post passes.
///
/// `this` is the subsystem object and `level` (float bits, transported
/// untouched) is the sync value. A busy byte at `+0x8eed8` is set while the
/// two worker routines run, then the node list at `+0x8ec50` is walked
/// (link at node `+0`): each node is synced with (`level`) and, when the
/// sync reports zero, drained. Afterwards the first post pass runs; the
/// second runs only when the low nibble of the global byte at file address
/// 0x01173604 is zero. The busy byte is cleared. The result is the last
/// post pass's answer.
///
/// Original: 0x00a9d940 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a9d940(this: u32, level: u32) -> u32 {
    unsafe {
        const BUSY_OFF: u32 = 0x8eed8;
        const LIST_OFF: u32 = 0x8ec50;
        const MODE_GLOBAL: u32 = 0x01173604;
        const MODE_MASK: u8 = 0x0f;
        const WORKER_A: u32 = 1;
        const WORKER_B: u32 = 2;
        const SYNC: u32 = 3;
        const DRAIN: u32 = 4;
        const POST_A: u32 = 5;
        const POST_B: u32 = 6;
        ((this + BUSY_OFF) as *mut u8).write(1);
        lf_checker_rt::callee_thiscall!(WORKER_A, u32, this);
        lf_checker_rt::callee_thiscall!(WORKER_B, u32, this);
        let mut node = ((this + LIST_OFF) as *const u32).read_unaligned();
        while node != 0 {
            let next = (node as *const u32).read_unaligned();
            let done: u32 = lf_checker_rt::callee_thiscall!(SYNC, u32, node, level);
            if done == 0 {
                lf_checker_rt::callee_thiscall!(DRAIN, u32, node);
            }
            node = next;
        }
        let mut answer: u32 = lf_checker_rt::callee_thiscall!(POST_A, u32, this);
        if lf_checker_rt::global::<u8>(MODE_GLOBAL).read_unaligned() & MODE_MASK == 0 {
            answer = lf_checker_rt::callee_thiscall!(POST_B, u32, this);
        }
        ((this + BUSY_OFF) as *mut u8).write(0);
        answer
    }
});
