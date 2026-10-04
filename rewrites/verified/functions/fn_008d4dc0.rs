// original: 0x008D4DC0 LockedCheckB
/// Runs the guarded check under the lock and returns its nonzero outcome.
/// The guard lives in the frame and is released on the way out.
export!(cdecl, rw_008D4DC0() -> u8 {
    unsafe {
        let mut guard = [0u32; 2];
        let gp = guard.as_mut_ptr() as u32;
        callee_thiscall!(0, u32, gp, relocated(0x11736fc));
        let ok = callee_cdecl!(1, u32,) as u8 != 0;
        callee_thiscall!(2, u32, gp);
        u8::from(ok)
    }
});
