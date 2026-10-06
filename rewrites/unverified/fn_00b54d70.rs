// original: 0x00B54D70 crmt_channel_open_d (proposed)

/// Resolve a channel with a fallback key, fill its context, then open it.
///
/// Like the sibling at 0x00B54D00 but guarded by a stack cookie and with
/// a middle step: after resolving (callee 1, else callee 2 with `a3`
/// unless it is -1) the filler (callee 3, thiscall on the context the
/// global at 0x16DD63C holds) writes three words through a frame slot,
/// and the opener (callee 4, thiscall on `this`) runs with the last two
/// of those words spliced into its arguments: (`handle`, word1, word2,
/// `a2`, `key`, `a1`, 0, 0, -1, -1). The frame slot address differs per
/// side and is skipped in favour of a snapshot. The cookie check (callee
/// 5) preserves registers. Returns zero on failure, else the opener's
/// answer.
///
/// Original: 0x00B54D70 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00b54d70(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const COOKIE_G: u32 = 0x1057fb4;
        const CTX_G: u32 = 0x16dd63c;
        const RESOLVE1: u32 = 1;
        const RESOLVE2: u32 = 2;
        const FILL: u32 = 3;
        const OPEN: u32 = 4;
        const COOKIE_CHECK: u32 = 5;
        const NONE: u32 = 0xffff_ffff;
        let cookie = lf_checker_rt::global::<u32>(COOKIE_G).read_unaligned();
        let _slot: u32 = cookie ^ 0x1234_5678;
        let mut key = a0;
        let mut handle: u32 = lf_checker_rt::callee_cdecl!(RESOLVE1, u32, a0, a1);
        if handle == 0 {
            if a3 == NONE {
                let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,);
                return 0;
            }
            key = a3;
            handle = lf_checker_rt::callee_cdecl!(RESOLVE2, u32, a3, a1);
            if handle == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,);
                return 0;
            }
        }
        let ctx = lf_checker_rt::global::<u32>(CTX_G).read_unaligned();
        let mut frame = [0u32; 3];
        let fp = (&mut frame as *mut u32) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(FILL, u32, ctx, key, a1, fp);
        let w1 = ((fp + 4) as *const u32).read_unaligned();
        let w2 = ((fp + 8) as *const u32).read_unaligned();
        let out: u32 = lf_checker_rt::callee_thiscall!(
            OPEN, u32, this, handle, w1, w2, a2, key, a1, 0, 0, NONE, NONE
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,);
        out
    }
});
