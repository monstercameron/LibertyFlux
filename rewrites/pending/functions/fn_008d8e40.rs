// original: 0x008D8E40 streaming_drain_and_release

/// Streaming drain-and-release: three pings, one report, tail call.
pub fn drain_and_release() -> u32 {
    unsafe {
        let mgr = relocated(0x011D4EC8);
        callee_thiscall!(0, u32, mgr);
        let v = callee_thiscall!(1, u32, mgr);
        callee_cdecl!(2, u32, v);
        callee_thiscall!(3, u32, mgr);
        callee_thiscall!(4, u32, mgr)
    }
}
