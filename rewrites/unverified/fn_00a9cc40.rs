// original: 0x00a9cc40 stream_drain_and_reset (proposed)

/// Drain every node, reset the three sub-tables, and clear the epoch.
///
/// `this` is the subsystem object. A busy byte at `+0x8eed9` is set while
/// the two worker routines run, then the node list at `+0x8ec50` is walked
/// (link at node `+0`) and each node is drained with the node in ECX. The
/// three sub-tables at `+0x8ec40`, `+0x8d830` and `+0x84820` are reset, the
/// notifier runs with the epoch word (global at file address 0x01394c10),
/// the epoch is cleared, the busy byte is cleared, and the function returns
/// nothing.
///
/// Original: 0x00a9cc40 (thiscall, no stack arguments; no result).
lf_checker_rt::export!(thiscall, rw_00a9cc40(this: u32) -> u32 {
    unsafe {
        const BUSY_OFF: u32 = 0x8eed9;
        const LIST_OFF: u32 = 0x8ec50;
        const EPOCH_GLOBAL: u32 = 0x01394c10;
        const WORKER_A: u32 = 1;
        const WORKER_B: u32 = 2;
        const DRAIN: u32 = 3;
        const RESET: u32 = 4;
        const NOTIFY: u32 = 5;
        ((this + BUSY_OFF) as *mut u8).write(1);
        lf_checker_rt::callee_thiscall!(WORKER_A, u32, this);
        lf_checker_rt::callee_thiscall!(WORKER_B, u32, this);
        let mut node = ((this + LIST_OFF) as *const u32).read_unaligned();
        while node != 0 {
            let next = (node as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(DRAIN, u32, node);
            node = next;
        }
        lf_checker_rt::callee_thiscall!(RESET, u32, this.wrapping_add(0x8ec40));
        lf_checker_rt::callee_thiscall!(RESET, u32, this.wrapping_add(0x8d830));
        lf_checker_rt::callee_thiscall!(RESET, u32, this.wrapping_add(0x84820));
        let epoch = lf_checker_rt::global::<u32>(EPOCH_GLOBAL).read_unaligned();
        lf_checker_rt::callee_cdecl!(NOTIFY, u32, epoch);
        lf_checker_rt::global::<u32>(EPOCH_GLOBAL).write_unaligned(0);
        ((this + BUSY_OFF) as *mut u8).write(0);
        0
    }
});
