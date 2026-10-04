// original: 0x0065fd20 rage::snHandleJoinRequestTask::vf2
/// Poll a join request: forward the pending invite while one is offered.
///
/// Marks the task started (`+0xc`), then, while the session state (`+0x50`
/// of `this+0x60`) is 2 or 3, asks the session for the pending request
/// (`this+0xd8`) and, when one is offered, hands its two words
/// (`+0xd0`/`+0xd4`) to the join dispatcher. When either step reports
/// nothing, or the session is in any other state, completes through
/// virtual slot 7 with `(0, 0)`. Returns the last callee answer.
export!(thiscall, rw_0065fd20(this: u32) -> u32 {
    unsafe {
        if ((this + 0xc) as *const u32).read() == 0 {
            ((this + 0xc) as *mut u32).write(1);
        }
        let session = ((this + 0x60) as *const u32).read();
        let state = ((session + 0x50) as *const u32).read();
        if state == 2 || state == 3 {
            let offered = callee_thiscall!(1, u32, session, this + 0xd8);
            if offered != 0 {
                let joined = callee_thiscall!(
                    2,
                    u32,
                    session,
                    ((this + 0xd0) as *const u32).read(),
                    ((this + 0xd4) as *const u32).read()
                );
                if joined != 0 {
                    return joined;
                }
            }
        }
        task_complete(this, 0, 0)
    }
});

