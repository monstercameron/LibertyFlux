// original: 0x008F9850 stream_channel_setup
/// Set up one streaming channel unless lane 0 is already active.
///
/// Probes lane 0; when it reports active, returns that result at
/// once. Otherwise runs the fourteen-word channel setup on this
/// object (both stack arguments carried through at fixed positions),
/// sets the done flag at `+0x9bd` and returns the setup's result.
/// Thiscall, two stack arguments.
export!(thiscall, rw_008f9850(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const DONE_FLAG: u32 = 0x9bd;
        let t: u32 = callee_stdcall!(1, u32, 0);
        if (t as u8) != 0 {
            return t;
        }
        let ans: u32 = callee_thiscall!(2, u32, this,
            a0, 0, 1, 0, 0, 0, a1, 0, 0, 0, 1, 0, 1, 0xFFFFFFFF);
        ((this + DONE_FLAG) as *mut u8).write(1);
        ans
    }
});
