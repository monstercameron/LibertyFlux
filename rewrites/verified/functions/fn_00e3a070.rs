
// original: 0x00e3a070 bind_input_source (proposed)

/// Bind a named input source from a descriptor object.
///
/// `arg` points to a descriptor holding a NUL-terminated name at `+0x1F8`
/// and a dword at `+0x324`. The name is copied to a frame buffer and its
/// length `len` is checked: `len - 4 >= 0x26` is compared UNSIGNED (`jae`)
/// and aborts via a noreturn stub (never taken on tested inputs, which keep
/// `len` in 4..41); otherwise the copy is truncated to `len - 4` chars.
/// A zero-length truncation (i.e. `len == 4`) takes the poll-and-teardown
/// path. The scratch length double below is `len2`, the truncated length.
///
/// Otherwise the poll flag at `cfg_a + 0x8C` (an equality compare against
/// 1, no signedness) optionally triggers an extra poll; `2 * (len2 + 1)`
/// bytes are allocated (the multiply's overflow arm is dead: lengths are
/// tiny, but the saturating form is replicated exactly) and freed again
/// after a convert and a format call; a build call and the path-classifier
/// (fn3) run over the copy; and a parse call writes an out-word. A zero
/// out-word (equality test) takes the short teardown path, otherwise a
/// virtual slot on the shared global object consumes the out-word and a
/// five-word descriptor `(0, 0, 0, 4, 0)`, and the returned handle is
/// installed: the old handle at `this + 0x234` (equality test against null)
/// is released, the new one stored with its refcount at `+0xA` bumped, and
/// the config at `this + 0x1E0` gets `-1` at `+0x1E0`, two zero words and
/// two 1.0 words at `+0x204..+0x213` (plain 8-byte copies, no float
/// arithmetic) and marker bytes at `+0x229/+0x22A`. A null allocation
/// returns 0 directly. The return value is the last callee's answer.
///
/// Original: 0x00E3A070 (thiscall, one stack word; 20 call sites).
lf_checker_rt::export!(thiscall, rw_00e3a070(this: u32, arg: u32) -> u32 {
    unsafe {
        const ARG_STR: u32 = 0x1F8;
        const ARG_DWORD: u32 = 0x324;
        const CFG_A: u32 = 0x22C;
        const CFG_B: u32 = 0x1E0;
        const SLOT234: u32 = 0x234;
        const POLL_FLAG: u32 = 0x8C;
        const GLOBAL_OBJ: u32 = 0x017F5630;
        const VT_SLOT: u32 = 8;
        const FMT1: u32 = 0x00F15108;
        const GLOB1: u32 = 0x0116A5D8;
        const FMT2: u32 = 0x00F1513C;
        const ONE_BITS: u32 = 0x3F800000;
        const OUT: u32 = 0x08;
        const VTBUF: u32 = 0x1C;
        const COPY: u32 = 0x30;
        const B58: u32 = 0x58;
        const B80: u32 = 0x80;
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
        const DONE: u32 = 16;
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
        unsafe fn strcpy(mut dst: u32, mut src: u32) {
            unsafe {
                loop {
                    let b = rd8(src);
                    wr8(dst, b);
                    src = src.wrapping_add(1);
                    dst = dst.wrapping_add(1);
                    if b == 0 {
                        break;
                    }
                }
            }
        }

        // One frame array at the original's offsets, so the copy's two-byte
        // overlap onto the build buffer at lengths 40..41 is identical.
        let mut frame = [0u32; 160];
        let fb = frame.as_mut_ptr() as u32;
        let outw = fb + OUT;
        let vtbuf = fb + VTBUF;
        let copy = fb + COPY;
        let b58 = fb + B58;
        let b80 = fb + B80;

        strcpy(copy, arg + ARG_STR);
        let len = strlen(copy);
        let t = len.wrapping_sub(4);
        if t >= 0x26 {
            // Length guard failed: the original jumps to a noreturn abort
            // stub. Unreachable on tested inputs (lengths 4..41).
            lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return 0xDEADBEEF;
        }
        wr8(copy + t, 0);
        let mut ans = lf_checker_rt::callee_cdecl!(FMTARG, u32, rd32(arg + ARG_DWORD));
        let cfg_a = rd32(this + CFG_A);
        if rd8(copy) == 0 {
            ans = lf_checker_rt::callee_thiscall!(POLL2, u32, cfg_a);
            ans = lf_checker_rt::callee_cdecl!(TEARDOWN, u32,);
            lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return ans;
        }
        if rd32(cfg_a + POLL_FLAG) == 1 {
            ans = lf_checker_rt::callee_thiscall!(POLL1, u32, cfg_a);
        }
        ans = lf_checker_rt::callee_thiscall!(POLL2, u32, cfg_a);
        let len2 = strlen(copy);
        let ebx = len2.wrapping_add(1);
        let prod = (ebx as u64) * 2;
        let nbytes = if prod > 0xFFFF_FFFF { 0xFFFF_FFFF } else { prod as u32 };
        let alloc = lf_checker_rt::callee_cdecl!(MALLOC, u32, nbytes);
        if alloc == 0 {
            lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return 0;
        }
        ans = lf_checker_rt::callee_cdecl!(CONVERT, u32, copy, alloc, ebx);
        ans = lf_checker_rt::callee_cdecl!(FORMAT, u32, b80, 0x200,
            lf_checker_rt::relocated(FMT1), lf_checker_rt::relocated(GLOB1), alloc);
        ans = lf_checker_rt::callee_thiscall!(POSTFMT, u32, cfg_a, b80);
        ans = lf_checker_rt::callee_cdecl!(FREE, u32, alloc);
        ans = lf_checker_rt::callee_thiscall!(BUILD, u32, this, copy, b58);
        ans = lf_checker_rt::callee_thiscall!(FN3, u32, this, copy);
        ans = lf_checker_rt::callee_cdecl!(LOOKUP2, u32, lf_checker_rt::relocated(FMT2), 0);
        wr32(outw, 0);
        ans = lf_checker_rt::callee_cdecl!(PARSE, u32, b58, outw);
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
        ans = lf_checker_rt::callee_thiscall!(DONE, u32, edx);
        ans = lf_checker_rt::callee_cdecl!(TEARDOWN, u32,);
        lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        ans
    }
});
