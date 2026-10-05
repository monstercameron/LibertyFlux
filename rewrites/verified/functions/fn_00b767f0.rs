// original: 0x00b767f0 anim_state_init (proposed)

/// Initialise the animation-state block at `this` from five arguments.
///
/// Stores the first word at `this+0xab4`, calls the setup callee (cdecl,
/// two stack words pushed as second argument then `this+0xbc4`, so the
/// callee sees destination first and value second; it pops nothing, so the
/// reads below still see the caller's five words), marks the block ready
/// (`this+0xbcc` = 1), then stores the third word at `this+0x990`, zero at
/// `this+0xaa0`, 150000 at `this+0x994`, the fourth word (a float, moved
/// bit-exact) at `this+0xbb8`, and the fifth word at `this+0xbbc`. Returns
/// the fifth word (the last value loaded into eax; the callee's answer is
/// ignored).
///
/// Original: 0x00b767f0 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00b767f0(this: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const FIRST_OFF: u32 = 0xab4;
        const SETUP_ARG_OFF: u32 = 0xbc4;
        const READY_OFF: u32 = 0xbcc;
        const THIRD_OFF: u32 = 0x990;
        const ZERO_OFF: u32 = 0xaa0;
        const COUNT_OFF: u32 = 0x994;
        const COUNT_VALUE: u32 = 150_000;
        const FOURTH_OFF: u32 = 0xbb8;
        const FIFTH_OFF: u32 = 0xbbc;
        const SETUP_CALLEE: u32 = 1;
        ((this + FIRST_OFF) as *mut u32).write_unaligned(a1);
        let _: u32 =
            lf_checker_rt::callee_cdecl!(SETUP_CALLEE, u32, this + SETUP_ARG_OFF, a2);
        ((this + READY_OFF) as *mut u8).write(1);
        ((this + THIRD_OFF) as *mut u32).write_unaligned(a3);
        ((this + ZERO_OFF) as *mut u32).write_unaligned(0);
        ((this + COUNT_OFF) as *mut u32).write_unaligned(COUNT_VALUE);
        ((this + FOURTH_OFF) as *mut u32).write_unaligned(a4);
        ((this + FIFTH_OFF) as *mut u32).write_unaligned(a5);
        a5
    }
});
