// original: 0x00621C80 net_join_gamers_apply (proposed)

/// Apply a join-gamers request: gate on the session id pair, then build
/// and dispatch the join (stages past the gate are not in this file yet).
///
/// `this` is the session manager; the five stack words are `arg1` (request
/// header, id pair at `+0/+4`), `arg2`, `arg3`, `arg4` (saved to the frame
/// but only `arg2` is observed on the stage-1 path) and `arg5`. The
/// original 8-aligns the stack pointer on entry; the rewrite models the
/// frame as a plain array, which is sound because stage 1 passes no frame
/// addresses anywhere (both calls take only heap pointers and integers).
///
/// Stage 1 (this file): the entry saves and the id gate. When either id
/// word differs, the fail tail runs: callee 1 (`0x622260`, thiscall/4)
/// with (`arg2`, 0, 0, 0) (`arg2` reaches index 0 through a push-shifted
/// slot), then the tail tests all take their entry-value branches (flag
/// byte 0, saved key -1, saved words 0) and the function ends with eax 0
/// through the third cookie check. The gate-match side panics here; it is
/// a later stage (see the lane's retry notes for the callee map).
///
/// Original: 0x00621C80 (thiscall, ecx = this, five stack words).
lf_checker_rt::export!(thiscall, rw_00621C80(
    this: u32,
    arg1: u32,
    arg2: u32,
    _arg3: u32,
    _arg4: u32,
    _arg5: u32,
) -> u32 {
    unsafe {
        const CB_DISP: u32 = 1;
        const CB_COOKIE: u32 = 2;

        const ID0_OFF: u32 = 0x540;
        const ID1_OFF: u32 = 0x544;

        const F_ARG1: u32 = 0x14;
        const F_ZERO_A: u32 = 0x1c;
        const F_ARG2: u32 = 0x28;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mut frame = [0u8; 0xb00];
        let fb = frame.as_mut_ptr() as u32;
        wr32(fb.wrapping_add(F_ARG1), arg1);
        wr32(fb.wrapping_add(F_ZERO_A), 0);
        wr32(fb.wrapping_add(F_ARG2), arg2);
        if rd32(this.wrapping_add(ID0_OFF)) != rd32(arg1)
            || rd32(this.wrapping_add(ID1_OFF)) != rd32(arg1.wrapping_add(4))
        {
            // Fail tail, all entry-value branches.
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CB_DISP,
                u32,
                this,
                rd32(fb.wrapping_add(F_ARG2)),
                0,
                0,
                0
            );
            let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
            return 0;
        }
        panic!("fn2 stage 2: gate-match side");
    }
});
