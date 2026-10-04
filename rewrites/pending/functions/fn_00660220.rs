// original: 0x00660220 rage::snConnectToPeerTask::vf7
/// Destroy a connect-to-peer task, flushing a pending request first.
///
/// Unless called for reason 1, unregisters the endpoint block at `+0xd0`
/// from the session registry (`+0x24` of `this+0x60`) and, when a request
/// id is still stored at `+0x2e0` (non-negative), cancels that request and
/// clears the slot. Always runs the base teardown with the same two
/// arguments and detaches the session. Returns the teardown answer.
export!(thiscall, rw_00660220(this: u32, reason: u32, flags: u32) -> u32 {
    unsafe {
        if reason != 1 {
            let session = ((this + 0x60) as *const u32).read();
            let registry = ((session + 0x24) as *const u32).read();
            callee_thiscall!(1, u32, registry, this + 0xd0);
            let pending = ((this + 0x2e0) as *const u32).read() as i32;
            if pending >= 0 {
                let registry = ((session + 0x24) as *const u32).read();
                callee_thiscall!(2, u32, registry, pending as u32, 1);
                ((this + 0x2e0) as *mut u32).write(0xffff_ffff);
            }
        }
        let out = callee_thiscall!(3, u32, this, reason, flags);
        ((this + 0x60) as *mut u32).write(0);
        out
    }
});

