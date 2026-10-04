// original: 0x00c59e30 task_timed_forward (proposed)

/// Forward an argument through the object's virtual slot when its stamp elapsed.
///
/// Returns the current timer at once when it has not passed the stamp at
/// `this+4`. Otherwise prepares through callee 1, then calls the virtual
/// slot at `+4` of the object's table with (`this`, `arg`) and returns its
/// answer. The indirect call lands on the checker's planted stub on both
/// sides.
///
/// Original: 0x00c59e30 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00c59e30(this: u32, arg: u32) -> u32 {
    unsafe {
        const TIMER_SLOT: u32 = 0x11735b4;
        const STAMP: u32 = 4;
        const VT_SLOT: u32 = 4;
        const PREPARE: u32 = 1;
        let timer = lf_checker_rt::global::<u32>(TIMER_SLOT).read();
        if timer <= ((this + STAMP) as *const u32).read_unaligned() {
            return timer;
        }
        lf_checker_rt::callee_thiscall!(PREPARE, u32, this, arg);
        let vt = (this as *const u32).read_unaligned();
        let tgt = ((vt + VT_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        f(this, arg)
    }
});

