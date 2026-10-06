
// original: 0x00e3a2f0 bind_input_name (proposed)

/// Bind a named input source from a NUL-terminated name.
///
/// Twin of fn_00e3a070 with a direct string argument: `name` is used in
/// place (no copy, no length guard, no truncation), and the format dword
/// comes from `this + 0x224`. A null or empty name (two equality tests)
/// takes the poll-only path.
///
/// The poll flag, allocation of `2 * (len + 1)` bytes (same dead-overflow
/// saturating multiply), convert, format, build, classifier and parse calls
/// match the twin. A zero out-word takes the short teardown path. On the
/// full path the installed handle and config writes match the twin, then
/// the out-word object's dword at `+0x20` is decremented: non-zero skips to
/// teardown, zero destroys it with the dtor callee and frees it. A null
/// allocation returns 0 directly. The return value is the last callee's
/// answer.
///
/// Original: 0x00E3A2F0 (thiscall, one stack word; 20 call sites).
lf_checker_rt::export!(thiscall, rw_00e3a2f0(this: u32, name: u32) -> u32 {
    unsafe {
        const THIS_DWORD: u32 = 0x224;
        const CFG_A: u32 = 0x22C;
        const CFG_B: u32 = 0x1E0;
        const SLOT234: u32 = 0x234;
        const POLL_FLAG: u32 = 0x8C;
        const GLOBAL_OBJ: u32 = 0x017F5630;
        const VT_SLOT: u32 = 8;
        const FMT1: u32 = 0x00F150C4;
        const GLOB1: u32 = 0x0116A5D8;
        const FMT2: u32 = 0x00F150F8;
        const ONE_BITS: u32 = 0x3F800000;
        const OUT: u32 = 0x08;
        const VTBUF: u32 = 0x1C;
        const B30: u32 = 0x30;
        const B80: u32 = 0x58;
        const FMTARG: u32 = 1;
        const POLL1: u32 = 2;
        const POLL2: u32 = 3;
        const MALLOC: u32 = 4;
        const CONVERT: u32 = 5;
        const FORMAT: u32 = 6;
        const POSTFMT: u32 = 7;
        const FREE: u32 = 8;
        const BUILD: u32 = 9;
        const FN3: u32 = 10;
        const LOOKUP2: u32 = 11;
        const PARSE: u32 = 12;
        const USE: u32 = 14;
        const RELEASE: u32 = 15;
        const DTOR: u32 = 16;
        const TEARDOWN: u32 = 17;
        const COOKIE: u32 = 18;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe fn strlen(mut s: u32) -> u32 {
            unsafe {
                let mut n = 0u32;
                while rd8(s) != 0 {
                    s = s.wrapping_add(1);
                    n = n.wrapping_add(1);
                }
                n
            }
        }

        let mut frame = [0u32; 160];
        let fb = frame.as_mut_ptr() as u32;
        let outw = fb + OUT;
        let vtbuf = fb + VTBUF;
        let b30 = fb + B30;
        let b80 = fb + B80;

        let cfg_a = rd32(this + CFG_A);
        if name == 0 || rd8(name) == 0 {
            let ans = lf_checker_rt::callee_thiscall!(POLL2, u32, cfg_a);
            lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return ans;
        }
        let mut ans;
        if rd32(cfg_a + POLL_FLAG) == 1 {
            ans = lf_checker_rt::callee_thiscall!(POLL1, u32, cfg_a);
        }
        ans = lf_checker_rt::callee_thiscall!(POLL2, u32, cfg_a);
        let ebp = strlen(name).wrapping_add(1);
        let prod = (ebp as u64) * 2;
        let nbytes = if prod > 0xFFFF_FFFF { 0xFFFF_FFFF } else { prod as u32 };
        let alloc = lf_checker_rt::callee_cdecl!(MALLOC, u32, nbytes);
        if alloc == 0 {
            lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return 0;
        }
        ans = lf_checker_rt::callee_cdecl!(CONVERT, u32, name, alloc, ebp);
        ans = lf_checker_rt::callee_cdecl!(FMTARG, u32, rd32(this + THIS_DWORD));
        ans = lf_checker_rt::callee_cdecl!(FORMAT, u32, b80, 0x200,
            lf_checker_rt::relocated(FMT1), lf_checker_rt::relocated(GLOB1), alloc);
        ans = lf_checker_rt::callee_thiscall!(POSTFMT, u32, cfg_a, b80);
        ans = lf_checker_rt::callee_cdecl!(FREE, u32, alloc);
        ans = lf_checker_rt::callee_thiscall!(BUILD, u32, this, name, b30);
        ans = lf_checker_rt::callee_thiscall!(FN3, u32, this, name);
        ans = lf_checker_rt::callee_cdecl!(LOOKUP2, u32, lf_checker_rt::relocated(FMT2), 0);
        wr32(outw, 0);
        ans = lf_checker_rt::callee_cdecl!(PARSE, u32, b30, outw);
        let edx = core::ptr::addr_of!(frame[(OUT / 4) as usize]).read_volatile();
        if edx == 0 {
            ans = lf_checker_rt::callee_cdecl!(TEARDOWN, u32,);
            lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return ans;
        }
        let gobj = rd32(lf_checker_rt::relocated(GLOBAL_OBJ));
        wr32(vtbuf, 0);
        wr32(vtbuf + 4, 0);
        wr32(vtbuf + 8, 0);
        wr32(vtbuf + 12, 4);
        wr32(vtbuf + 16, 0);
        let vt: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(gobj) + VT_SLOT) as usize);
        let edi = vt(gobj, edx, vtbuf);
        let cfg_b = rd32(this + CFG_B);
        ans = lf_checker_rt::callee_thiscall!(USE, u32, cfg_b, edi);
        wr32(cfg_b + 0x1E0, 0xFFFF_FFFF);
        wr32(cfg_b + 0x204, 0);
        wr32(cfg_b + 0x208, 0);
        wr32(cfg_b + 0x20C, ONE_BITS);
        wr32(cfg_b + 0x210, ONE_BITS);
        let slot = rd32(this + SLOT234);
        if slot != 0 {
            ans = lf_checker_rt::callee_thiscall!(RELEASE, u32, slot);
            wr32(this + SLOT234, 0);
        }
        wr32(this + SLOT234, edi);
        wr16(edi + 0x0A, rd16(edi + 0x0A).wrapping_add(1));
        let cfg_b2 = rd32(this + CFG_B);
        wr8(cfg_b2 + 0x229, 1);
        wr8(cfg_b2 + 0x22A, 1);
        let rc = rd32(edx + 0x20).wrapping_sub(1);
        wr32(edx + 0x20, rc);
        if rc != 0 {
            ans = lf_checker_rt::callee_cdecl!(TEARDOWN, u32,);
            lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return ans;
        }
        ans = lf_checker_rt::callee_thiscall!(DTOR, u32, edx);
        ans = lf_checker_rt::callee_cdecl!(FREE, u32, edx);
        ans = lf_checker_rt::callee_cdecl!(TEARDOWN, u32,);
        lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        ans
    }
});
