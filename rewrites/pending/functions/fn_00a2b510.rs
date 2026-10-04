// original: 0x00a2b510 CPlayerPed::vf60

/// Store a float parameter and forward it to the parameter worker.
/// Writes the argument bits to `+0xE9C` of the object, then calls the
/// worker (callee 1, thiscall with this and the float bits as one stack
/// word, callee cleans the stack) and returns the worker's answer.
/// Original: 0x00a2b510 (thiscall, one float stack word).
lf_checker_rt::export!(thiscall, rw_00a2b510(this: u32, fbits_arg: u32) -> u32 {
    unsafe {
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
        const PARAM: u32 = 0xe9c;
        const WORKER: u32 = 1;
        wr32(this.wrapping_add(PARAM), fbits_arg);
        lf_checker_rt::callee_thiscall!(WORKER, u32, this, fbits_arg)
    }
});
