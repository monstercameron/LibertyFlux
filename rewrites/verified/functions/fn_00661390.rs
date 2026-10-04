// original: 0x00661390 rage::snLeaveGamersFromRlineTask::vf7
/// Destroy a leave-gamers task, unregistering an active leave first.
///
/// When a leave is in flight (`+0x94` is 1), unregisters it from the
/// session registry (`+0x48` of `this+0x60`). Always runs the base
/// teardown with the same two arguments and detaches the session.
/// Returns the teardown answer.
export!(thiscall, rw_00661390(this: u32, reason: u32, flags: u32) -> u32 {
    unsafe {
        if ((this + 0x94) as *const u32).read() == 1 {
            let session = ((this + 0x60) as *const u32).read();
            callee_thiscall!(1, u32, session + 0x48, this + 0x94);
        }
        let out = callee_thiscall!(2, u32, this, reason, flags);
        ((this + 0x60) as *mut u32).write(0);
        out
    }
});

