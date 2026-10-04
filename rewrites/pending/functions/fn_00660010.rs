// original: 0x00660010 rage::snConnectToPeerTask::snConnectToPeerTask_2
/// Tear down a connect-to-peer task: endpoint, members, chained cleanup.
///
/// Reinstalls this class's vtable, then reads the endpoint state at
/// `+0x288`: for states 1..=9 the live endpoint handle (`+0x140`) is
/// released first. Always destroys the endpoint member at `+0xd0`,
/// switches to the base vtable, and, when the chain word at `+0x18` is
/// non-null, runs the chained cleanup on `this`. Returns the last answer.
export!(thiscall, rw_00660010(this: u32) -> u32 {
    unsafe {
        (this as *mut u32).write(relocated(0xfe32c0));
        let endpoint_state = ((this + 0x288) as *const u32).read();
        if (1..=9).contains(&endpoint_state) {
            callee_thiscall!(1, u32, ((this + 0x140) as *const u32).read(), this + 0xd0);
        }
        let destroyed = callee_thiscall!(2, u32, this + 0xd0);
        let chained = ((this + 0x18) as *const u32).read();
        (this as *mut u32).write(relocated(0xfe4d60));
        if chained != 0 {
            callee_thiscall!(3, u32, chained, this)
        } else {
            destroyed
        }
    }
});

