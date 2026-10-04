// original: 0x00E6F950 timer_queue_drain_std
/// Drain the stdcall timer queue through the engine release hook.
///
/// Repeatedly passes the queue head to the release hook until the queue
/// reports empty. Does nothing when the queue is already empty.
export!(cdecl, rw_00e6f950() -> u32 {
    unsafe {
        const HEAD: u32 = 0x019F3224;
        const COUNT: u32 = 0x019F322C;
        while *global::<u32>(COUNT) != 0 {
            let head = *global::<u32>(HEAD);
            if head != 0 {
                callee_stdcall!(1, u32, head);
            }
        }
        0
    }
});
