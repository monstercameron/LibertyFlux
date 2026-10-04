// original: 0x00E6FBA0 timer_queue_drain_b
/// Drain timer queue B through the engine release hook.
///
/// Repeatedly passes the queue head to the release hook until the queue
/// reports empty. Does nothing when the queue is already empty.
export!(cdecl, rw_00e6fba0() -> u32 {
    unsafe {
        const HEAD: u32 = 0x019F7F2C;
        const COUNT: u32 = 0x019F7F34;
        if *global::<u32>(COUNT) == 0 {
            return 0;
        }
        while *global::<u32>(COUNT) != 0 {
            let head = *global::<u32>(HEAD);
            if head != 0 {
                callee_thiscall!(1, u32, relocated(HEAD), head);
            }
        }
        0
    }
});
