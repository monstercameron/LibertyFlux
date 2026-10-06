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
/// Stage 1: the entry saves and the id gate. When either id word
/// differs, the fail tail runs: callee 1 (`0x622260`, thiscall/4) with
/// (`arg2`, 0, 0, 0) (`arg2` reaches index 0 through a push-shifted
/// slot), then the tail tests all take their entry-value branches (flag
/// byte 0, saved key -1, saved words 0) and the function ends with eax 0
/// through the third cookie check.
///
/// Stage 2 (this file): the gate-match prefix. Callee 3 (`0x623490`,
/// thiscall/1) answers the join task (null ends like stage 1); callee 4
/// (`0x6221e0`, thiscall/1) copies the request header to frame scratch;
/// 0x80 words move from `arg1+0x298` to the frame; callee 5 (`0x622f60`,
/// thiscall/1) searches the slots (miss); callee 6 (`0x6212b0`,
/// thiscall/1) counts them (zero); the empty slot ends through the
/// dispatch (index 1 now 2) and the release fastcall (callee 7,
/// `0x6264d0`, no stack arguments) with the saved task pointer. Branches
/// needing a nonzero search, count or slot (the rest of the function)
/// fault here; they are stage 3 (see the lane's retry notes).
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
        const CB_JOIN: u32 = 3;
        const CB_HDR: u32 = 4;
        const CB_FIND: u32 = 5;
        const CB_COUNT: u32 = 6;
        const CB_REL: u32 = 7;

        const ID0_OFF: u32 = 0x540;
        const ID1_OFF: u32 = 0x544;
        const ARGSAVE_OFF: u32 = 0x290;
        const COPY_SRC_OFF: u32 = 0x298;
        const COPY_WORDS: u32 = 0x80;

        const F_ARG1: u32 = 0x14;
        const F_ZERO_A: u32 = 0x1c;
        const F_ARG2: u32 = 0x28;
        const F_FLAG: u32 = 0x0f;
        const F_SAVED0: u32 = 0x1c;
        const F_HDR: u32 = 0x38;
        const F_COPY0: u32 = 0x2c8;
        const F_COPYDST: u32 = 0x2d0;
        const F_FINDARG: u32 = 0x88;
        const F_SLOT: u32 = 0xc0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        // Gate-match side (stage 2): join task, header copy, block copy,
        // slot search (miss), slot count (zero), empty-slot end.
        let j: u32 = lf_checker_rt::callee_thiscall!(
            CB_JOIN,
            u32,
            this,
            fb.wrapping_add(F_FLAG)
        );
        wr32(fb.wrapping_add(F_SAVED0), j);
        if j == 0 {
            // Same fail tail as the gate mismatch (saved word is 0).
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
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CB_HDR,
            u32,
            fb.wrapping_add(F_HDR),
            arg1
        );
        wr32(
            fb.wrapping_add(F_COPY0),
            rd32(arg1.wrapping_add(ARGSAVE_OFF)),
        );
        let mut s = arg1.wrapping_add(COPY_SRC_OFF);
        let mut d = fb.wrapping_add(F_COPYDST);
        let mut n = COPY_WORDS;
        while n != 0 {
            wr32(d, rd32(s));
            s = s.wrapping_add(4);
            d = d.wrapping_add(4);
            n -= 1;
        }
        let f: u32 = lf_checker_rt::callee_thiscall!(
            CB_FIND,
            u32,
            this,
            fb.wrapping_add(F_FINDARG)
        );
        if f != 0 {
            rd32(0); // stage 3: search hit.
        }
        let slot = rd32(fb.wrapping_add(F_SLOT));
        let c: u32 =
            lf_checker_rt::callee_thiscall!(CB_COUNT, u32, this, slot);
        if c != 0 {
            rd32(0); // stage 3: nonzero count.
        }
        if slot != 0 {
            rd32(0); // stage 3: live slot scan.
        }
        // Empty-slot end: dispatch takes 2, then the release fastcall.
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CB_DISP,
            u32,
            this,
            rd32(fb.wrapping_add(F_ARG2)),
            2,
            0,
            0
        );
        if rd8(fb.wrapping_add(F_FLAG)) != 0 {
            rd32(0); // stage 3: flag set (needs callee writes modelled).
        }
        let rel: u32 = lf_checker_rt::callee_fastcall!(
            CB_REL,
            u32,
            rd32(fb.wrapping_add(F_SAVED0)),
            rd32(this)
        );
        let _ = rel;
        let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
        return 0;
    }
});
