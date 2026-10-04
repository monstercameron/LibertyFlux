// original: 0x00E6FB70 timer_queue_drain_a
/// Drain timer queue A through the engine release hook.
///
/// Repeatedly passes the queue head to the release hook until the queue
/// reports empty. Does nothing when the queue is already empty.
export!(cdecl, rw_00e6fb70() -> u32 {
    unsafe {
        const HEAD: u32 = 0x019F3A68;
        const COUNT: u32 = 0x019F3A70;
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
