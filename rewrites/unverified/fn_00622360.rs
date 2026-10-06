// original: 0x00622360 rage::rlFireAndForgetTask<rage::snDropGamersTask>::snDropGamersTask> (symbols)

/// Session drop-gamers task setup: when the session is live and the id
/// pair matches, check every gamer slot and dispatch the drop; otherwise
/// take the mismatch path (stage 2, not in this file yet).
///
/// `this` is the session manager, `arg1` the request (null ends
/// immediately), `arg2` the gamer list (count at `+0x10c`, entries of 8
/// bytes from `+8`). When `this+0x50` is 2 or 3 (compared SIGNED with
/// `jl`/`jg`; both miss directions land on the same path, so no wrong
/// version can distinguish the signedness here) and the id words at
/// `this+0xbf0`/`+0xbf4` equal those at `+0xc30`/`+0xc34`, each entry is
/// checked by callee 1 (`0x61f090`, thiscall/1 with `arg1`); the count of
/// successes must equal the entry count (compared for equality, no
/// signedness), else the function ends. The entry count gate itself is
/// SIGNED (`jle`): 0 skips the loop and continues, negatives end.
///
/// On success the frame scratch `[+0x38, +0x228)` is zeroed, callee 2
/// (`0x6211c0`, thiscall/4) copies address blocks into it, and callee 3
/// (`0x623880`, thiscall/7) dispatches. Both take the scratch at `+0x38`
/// through push-shifted `lea`s. Callee 3's fifth stack word is whatever
/// the callee-2 stub left in ecx (clobbered scratch, compared nowhere).
/// The security-cookie computation is frame scratch feeding only the
/// stubbed check (id 4) and is not modelled. Both direct callers ignore
/// eax, so the contract compares no return channel.
///
/// Otherwise (state outside 2/3, or the id words differing) the
/// request's own id pair (at `arg1+0x40`/`+0x44`) is compared against the
/// wanted pair; when it differs too, the mismatch tail (stage 2a) checks
/// entries until the first success and reports it through callee 19
/// (`0x620b80`, thiscall/2). When the request pair matches, the match
/// side (stage 2b: find-scan, chain walks, task build) panics here.
///
/// Original: 0x00622360 (thiscall, ecx = this, two stack words).
lf_checker_rt::export!(thiscall, rw_00622360(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const CB_CHECK: u32 = 1;
        const CB_COPY: u32 = 2;
        const CB_DISPATCH: u32 = 3;
        const CB_COOKIE: u32 = 4;
        const CB_TAIL: u32 = 19;

        const STATE_OFF: u32 = 0x50;
        const ID0_OFF: u32 = 0xbf0;
        const ID1_OFF: u32 = 0xbf4;
        const WANT0_OFF: u32 = 0xc30;
        const WANT1_OFF: u32 = 0xc34;
        const COUNT_OFF: u32 = 0x10c;
        const ENTRIES_OFF: u32 = 0x08;
        const ENTRY_STRIDE: u32 = 8;
        const EXTRA_OFF: u32 = 0x108;

        const F_ARG2: u32 = 0x14;
        const F_ARG1: u32 = 0x1c;
        const F_THIS: u32 = 0x20;
        const F_COUNT: u32 = 0x18;
        const F_SCRATCH: u32 = 0x38;
        const F_ZERO_BASE: u32 = 0x41;
        const F_ZERO_ITERS: i32 = 31;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        let mut frame = [0u8; 0x244];
        let fb = frame.as_mut_ptr() as u32;
        wr32(fb.wrapping_add(F_THIS), this);
        wr32(fb.wrapping_add(F_ARG1), arg1);
        wr32(fb.wrapping_add(F_ARG2), arg2);
        if arg1 == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
            return 0;
        }
        // Signed state gate; both misses share the path-2 target.
        let state = rd32(this.wrapping_add(STATE_OFF)) as i32;
        if state < 2
            || state > 3
            || rd32(this.wrapping_add(ID0_OFF)) != rd32(this.wrapping_add(WANT0_OFF))
            || rd32(this.wrapping_add(ID1_OFF)) != rd32(this.wrapping_add(WANT1_OFF))
        {
            // Path 2: compare the request's id pair, then the mismatch tail
            // (stage 2a). The match side is stage 2b.
            let d = rd32(arg1.wrapping_add(0x40));
            let a = rd32(arg1.wrapping_add(0x44));
            wr32(fb.wrapping_add(F_ARG2), d);
            wr32(fb.wrapping_add(F_COUNT), a);
            if d != rd32(this.wrapping_add(WANT0_OFF))
                || a != rd32(this.wrapping_add(WANT1_OFF))
            {
                let n = rd32(arg2.wrapping_add(COUNT_OFF));
                if (n as i32) > 0 {
                    let mut e = arg2.wrapping_add(ENTRIES_OFF);
                    let mut i = 0u32;
                    loop {
                        let r: u32 =
                            lf_checker_rt::callee_thiscall!(CB_CHECK, u32, arg1, e);
                        if r != 0 {
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                CB_TAIL,
                                u32,
                                this,
                                rd32(fb.wrapping_add(F_ARG2)),
                                rd32(fb.wrapping_add(F_COUNT))
                            );
                            break;
                        }
                        i += 1;
                        if i >= n {
                            break;
                        }
                        e = e.wrapping_add(ENTRY_STRIDE);
                    }
                }
                let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
                return 0;
            }
            panic!("fn3 stage 2b: match path");
        }
        let count = rd32(arg2.wrapping_add(COUNT_OFF));
        wr32(fb.wrapping_add(F_COUNT), count);
        let mut ok = 0u32;
        // Signed count gate: jle skips the loop for 0 and negatives.
        if (count as i32) > 0 {
            let mut e = arg2.wrapping_add(ENTRIES_OFF);
            let mut left = count;
            loop {
                let r: u32 = lf_checker_rt::callee_thiscall!(CB_CHECK, u32, arg1, e);
                if r != 0 {
                    ok += 1;
                }
                e = e.wrapping_add(ENTRY_STRIDE);
                left = left.wrapping_sub(1);
                if left == 0 {
                    break;
                }
            }
        }
        if count != ok {
            let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
            return 0;
        }
        let mut z = F_ZERO_BASE;
        let mut cc = F_ZERO_ITERS;
        loop {
            cc -= 1;
            wr64(fb.wrapping_add(z).wrapping_sub(9), 0);
            wr64(fb.wrapping_add(z).wrapping_sub(1), 0);
            wr16(fb.wrapping_add(z).wrapping_sub(1), 0);
            z = z.wrapping_add(0x10);
            if cc < 0 {
                break;
            }
        }
        // idx0 is the last pushed word: entries, count, scratch, -1.
        let copied: u32 = lf_checker_rt::callee_thiscall!(
            CB_COPY,
            u32,
            this,
            arg2.wrapping_add(ENTRIES_OFF),
            count,
            fb.wrapping_add(F_SCRATCH),
            0xFFFF_FFFF
        );
        // idx0..idx6: arg1, scratch, copied, 0, ecx-residue, extra, 0.
        // idx4 is the copy call's clobbered ecx; the contract skips it.
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CB_DISPATCH,
            u32,
            this,
            arg1,
            fb.wrapping_add(F_SCRATCH),
            copied,
            0,
            0,
            rd32(arg2.wrapping_add(EXTRA_OFF)),
            0
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
        0
    }
});
