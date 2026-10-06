// original: 0x00e3a530 RP_EXPORTHI (symbols)

/// Classify an input path into a mode and report it.
///
/// `path` is a NUL-terminated string or null. A local two-word scratch
/// `{buf, len}` starts zeroed; when `path` is non-null the init callee is
/// given its length and address and fills the scratch (the two words are
/// compared at call time; the address itself is a frame pointer and is
/// skipped). The scratch length is used through its low 16 bits only.
///
/// Block A looks up a suffix string and checks whether the scratch buffer
/// ends with it: the suffix length `la` and the scratch length's low 16
/// bits `llen` are compared SIGNED (`jg`; both are non-negative lengths, a
/// strlen and a u16, so an unsigned compare would be identical and the
/// signedness is unobservable), and on `la <= llen` the last `la` bytes of
/// `buf[0..llen]` are compared against the suffix from the end down, with
/// an empty suffix (`dec; js`) always matching. On a match the mode word at
/// `this+0x23C` becomes 3. Otherwise block B looks up a second string and
/// the equals callee tests it against the scratch (the low byte of its
/// answer decides); non-zero sets mode 2. Otherwise block C tests a third
/// string and sets mode 1 or 0 from whether its answer's low byte is
/// non-zero. Each path ends by calling the setter callee with `buf`.
///
/// Original: 0x00E3A530 (thiscall, one stack word; 12 direct call sites).
lf_checker_rt::export!(thiscall, rw_00e3a530(this: u32, path: u32) -> u32 {
    unsafe {
        const CFG_OBJECT: u32 = 0x0116BFF0;
        const KEY_SUFFIX: u32 = 0x00F143F8;
        const KEY_B: u32 = 0x00F14404;
        const KEY_C: u32 = 0x00F14414;
        const MODE: u32 = 0x23C;
        const INIT: u32 = 1;
        const LOOKUP: u32 = 2;
        const GET_A: u32 = 3;
        const GET_B: u32 = 4;
        const EQ_B: u32 = 5;
        const GET_C: u32 = 6;
        const EQ_C: u32 = 7;
        const SETTER: u32 = 8;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        unsafe fn lookup(key: u32) -> u32 {
            unsafe {
                // Stack order as the original pushes: 0, 0, key.
                lf_checker_rt::callee_thiscall!(LOOKUP, u32,
                    lf_checker_rt::relocated(CFG_OBJECT),
                    lf_checker_rt::relocated(key), 0, 0)
            }
        }

        let mut local = [0u32; 2];
        if path != 0 {
            let n = strlen(path);
            // Stack order as the original pushes: len, then the string.
            lf_checker_rt::callee_thiscall!(INIT, u32, local.as_mut_ptr() as u32, path, n);
            // The stub wrote the scratch through the pointer; reload it.
            local[0] = core::ptr::addr_of!(local[0]).read_volatile();
            local[1] = core::ptr::addr_of!(local[1]).read_volatile();
        }
        let (buf, lenw) = (local[0], local[1]);
        // Block A: does the scratch buffer end with the looked-up suffix?
        let sa = lf_checker_rt::callee_stdcall!(GET_A, u32, lookup(KEY_SUFFIX));
        let la = strlen(sa);
        let llen = lenw & 0xFFFF;
        let mut matched = false;
        if !((la as i32) > (llen as i32)) {
            if la == 0 {
                matched = true;
            } else {
                let base = llen.wrapping_sub(la);
                let mut i = la;
                let mut ok = true;
                while i > 0 {
                    i -= 1;
                    if rd8(buf.wrapping_add(base).wrapping_add(i)) != rd8(sa.wrapping_add(i)) {
                        ok = false;
                        break;
                    }
                }
                matched = ok;
            }
        }
        if matched {
            wr32(this + MODE, 3);
            return lf_checker_rt::callee_cdecl!(SETTER, u32, buf);
        }
        // Block B: the equals callee tests the second string.
        let sb = lf_checker_rt::callee_stdcall!(GET_B, u32, lookup(KEY_B));
        let eb = lf_checker_rt::callee_thiscall!(EQ_B, u32, local.as_mut_ptr() as u32, sb);
        if (eb as u8) != 0 {
            wr32(this + MODE, 2);
            return lf_checker_rt::callee_cdecl!(SETTER, u32, buf);
        }
        // Block C: the equals callee tests the third string; mode is 1/0.
        let sc = lf_checker_rt::callee_stdcall!(GET_C, u32, lookup(KEY_C));
        let ec = lf_checker_rt::callee_thiscall!(EQ_C, u32, local.as_mut_ptr() as u32, sc);
        wr32(this + MODE, u32::from((ec as u8) != 0));
        lf_checker_rt::callee_cdecl!(SETTER, u32, buf)
    }
});
