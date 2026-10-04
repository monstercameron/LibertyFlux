// original: 0x00a1e830 cam_state_dispatch_copy (proposed)

/// Dispatches on the state word: initialise, copy out, or delegate.
///
/// `this` holds a state word at `+STATE_OFF`. When it is -1 the
/// initialiser callee runs first (its answer is ignored). A state of 0
/// copies the triple at `+TRI_OFF` to `a0[0..12]`, replicates the float
/// at `+REP_OFF` to `a0+0x10/0x14/0x18`, zeroes `a0+0x50` and returns. A
/// state of 4 delegates to the worker callee on (`a0`, `this+ARG0_OFF`,
/// `this+ARG1_OFF`). Any other state returns without doing anything.
/// Returns nothing.
///
/// Original: 0x00a1e830 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a1e830(this: u32, a0: u32) -> u32 {
    unsafe {
        const C_INIT: u32 = 1;
        const C_WORKER: u32 = 2;
        const STATE_OFF: u32 = 0x494;
        const TRI_OFF: u32 = 0x420;
        const REP_OFF: u32 = 0x490;
        const ARG0_OFF: u32 = 0x440;
        const ARG1_OFF: u32 = 0x480;
        const CLEAR_OFF: u32 = 0x50;
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        if rd32(this + STATE_OFF) == 0xffff_ffff {
            lf_checker_rt::callee_thiscall!(C_INIT, u32, this);
        }
        match rd32(this + STATE_OFF) {
            0 => {
                ((a0) as *mut u32).write_unaligned(rd32(this + TRI_OFF));
                ((a0 + 4) as *mut u32).write_unaligned(rd32(this + TRI_OFF + 4));
                ((a0 + 8) as *mut u32).write_unaligned(rd32(this + TRI_OFF + 8));
                let r = rd32(this + REP_OFF);
                ((a0 + 0x18) as *mut u32).write_unaligned(r);
                ((a0 + 0x14) as *mut u32).write_unaligned(r);
                ((a0 + 0x10) as *mut u32).write_unaligned(r);
                ((a0 + CLEAR_OFF) as *mut u32).write_unaligned(0);
            }
            4 => {
                lf_checker_rt::callee_thiscall!(C_WORKER, u32, a0, this + ARG0_OFF, this + ARG1_OFF);
            }
            _ => {}
        }
        0
    }
});
