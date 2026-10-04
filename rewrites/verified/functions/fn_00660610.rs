// original: 0x00660610 rage::snAddRemoteGamerTask::vf2
/// Poll an add-gamer request: complete once the session offers a gamer.
///
/// Marks the task started (`+0xc`). While the session state (`+0x50` of
/// `this+0x60`) is 2 or 3, asks the session for the pending gamer
/// (`this+0xe0`): when one is offered the task completes through virtual
/// slot 7 with `(0, 0)`; when none is offered it returns 0 and is polled
/// again later. Outside states 2..=3 it completes immediately.
export!(thiscall, rw_00660610(this: u32) -> u32 {
    unsafe {
        if ((this + 0xc) as *const u32).read() == 0 {
            ((this + 0xc) as *mut u32).write(1);
        }
        let session = ((this + 0x60) as *const u32).read();
        let state = ((session + 0x50) as *const u32).read();
        if state != 2 && state != 3 {
            return task_complete(this, 0, 0);
        }
        let offered = callee_thiscall!(1, u32, session, this + 0xe0);
        if offered == 0 {
            0
        } else {
            task_complete(this, 0, 0)
        }
    }
});

