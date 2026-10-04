// original: 0x00661c10 rage::snEstablishSessionTask::vf7
/// Establish-session task teardown: settle state, refresh, forward.
///
/// thiscall/2, returns void. When a result is pending, settles it through the
/// handler the latched state selects (query, publish or drain); unless
/// already finished, refreshes the session entry and clears a raised refresh
/// flag; then forwards both arguments to the shared teardown helper and
/// clears the session link.
export!(thiscall, rw_00661c10(this_ptr: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        if ((this_ptr + 0x94) as *const u32).read() == 1 {
            match ((this_ptr + 0x90) as *const u32).read() {
                1 => {
                    callee_thiscall!(1, u32, this_ptr.wrapping_add(0x94));
                }
                3 | 5 | 9 => {
                    let session = ((this_ptr + 0x60) as *const u32).read();
                    callee_thiscall!(3, u32, session.wrapping_add(0x48),
                        this_ptr.wrapping_add(0x94));
                }
                7 => {
                    let session = ((this_ptr + 0x60) as *const u32).read();
                    let mgr = ((session + 0x24) as *const u32).read();
                    callee_thiscall!(2, u32, mgr, this_ptr.wrapping_add(0x768));
                }
                _ => {}
            }
        }
        // The refresh block below, including the pending-flag check, runs
        // only when not already finished; otherwise control jumps past it.
        if a0 != 1 {
            let lo = ((this_ptr + 0x120) as *const u32).read();
            let hi = ((this_ptr + 0x124) as *const u32).read();
            let session = ((this_ptr + 0x60) as *const u32).read();
            let first: u32 = callee_thiscall!(4, u32, session, lo, hi);
            if first != 0 {
                let second: u32 = callee_thiscall!(5, u32, session, lo, hi);
                if second != 0 {
                    callee_thiscall!(6, u32, session, second);
                }
            }
            let session = ((this_ptr + 0x60) as *const u32).read();
            let pending = (session + 0x32f8) as *mut u8;
            if pending.read() & 1 != 0 {
                callee_thiscall!(7, u32, session.wrapping_add(0xc6c));
                let mgr = ((session + 0x24) as *const u32).read();
                callee_thiscall!(8, u32, mgr, session.wrapping_add(0x28));
                pending.write(pending.read() & 0xfe);
            }
        }
        callee_thiscall!(9, u32, this_ptr, a0, a1);
        ((this_ptr + 0x60) as *mut u32).write(0);
        0
    }
});
