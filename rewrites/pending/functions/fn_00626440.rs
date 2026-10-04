// original: 0x00626440 guarded_frame_probe
// Run a two-phase probe through a scratch frame under a stack-cookie guard.
//
// Fills the frame through the first call, feeds the produced word together
// with the leading argument into the second call, and runs the cookie check
// on every exit. Any failed phase skips the later calls; every path returns
// the cookie check's answer since that call runs last. The cookie value
// mixes the stack pointer so it legitimately differs between the two frames
// and is passed as 0 with its register skipped.
export!(thiscall, rw_00626440(holder: u32, a0: u32, a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const PHASE_TAG: u32 = 0x394;
        let mut frame = [0u32; 236];
        let base = frame.as_mut_ptr();
        let p1 = base as u32;
        let p2 = (base.add(1)) as u32;
        let first: u32 = callee_thiscall!(1, u32, a1, p2, PHASE_TAG, p1);
        if first & 0xff == 0 {
            return callee_thiscall!(3, u32, 0);
        }
        let produced: u32 = *(p1 as *const u32);
        let second: u32 = callee_thiscall!(2, u32, holder, a0, p2, produced, 1, 0);
        if second & 0xff == 0 {
            return callee_thiscall!(3, u32, 0);
        }
        callee_thiscall!(3, u32, 0)
    }
});
