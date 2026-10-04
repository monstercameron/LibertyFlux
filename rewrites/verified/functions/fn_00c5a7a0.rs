// original: 0x00c5a7a0 task_try_then_forward (proposed)

/// Try callee 1 first, forwarding through the virtual slot when it vetoes.
///
/// Invokes callee 1 with (`this`, `a0`, `a1`); when its answer's low byte is
/// nonzero that answer is returned. Otherwise the virtual slot at `+0xc` of
/// the object's table is called with (`this`, `a0`, `a1`) and its answer is
/// returned. The indirect call lands on the checker's planted stub on both
/// sides.
///
/// Original: 0x00c5a7a0 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_00c5a7a0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const VT_SLOT: u32 = 0x0c;
        const TRY: u32 = 1;
        let r: u32 = lf_checker_rt::callee_thiscall!(TRY, u32, this, a0, a1);
        if (r & 0xff) != 0 {
            return r;
        }
        let vt = (this as *const u32).read_unaligned();
        let tgt = ((vt + VT_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        f(this, a0, a1)
    }
});

