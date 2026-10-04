
// original: 0x00E4CA30 video_export_apply_quality_arg (proposed)

/// Twin of rw_00E4CC60 that takes its clip name from a sibling object
/// instead of a bare string: copy the name at `arg+0x1f8` through a scratch
/// buffer, strip its last four characters, and store it on the panel's name
/// field (`this+0x209`), with the quality at `arg+0x324` mirrored to
/// `this+0x430` and to the child object's `+0x224` slot.
///
/// Names shorter than 4 or longer than 41 characters take the original's
/// overflow path (a stub call, then a breakpoint trap). That trap cannot be
/// reproduced without assembly, so the contract keeps every trial in range
/// and this rewrite faults (null read) instead of returning there, so a
/// widened contract would fail honestly rather than pass wrongly.
///
/// Past the copy the behaviour matches rw_00E4CC60 except that the three
/// suffix tests are callee calls: callee 3/4/5 each take the converted
/// (pointer, length) pair plus one stored string and answer whether it
/// matches; on a match with the converted length above the pair's strip
/// length the tail is cut and the string re-terminated. The surviving string
/// is published through virtual slot `+0x1e0` of `this+0x1ec`, callee 6
/// classifies the stored name into a quality label ("RP_EXPORTLOW",
/// "RP_EXPORTMED", "RP_EXPORTHI", lengths 12, 12, 11; any other answer keeps
/// the shared empty string), callee 7 converts the label, slot `+0x1e0` of
/// `this+0x1f4` publishes it, and callee 8 releases both buffers, label
/// first. Callee 10 is the cookie check (register-preserving).
///
/// Returns callee 8's answer for the name buffer. Original: 0x00E4CA30
/// (thiscall, one stack word). Callee ids: 1 child sync, 2 convert (name),
/// 3/4/5 suffix tests, 6 classify, 7 convert (label), 8 release, 9 virtual
/// slot +0x1e0, 10 cookie check.
lf_checker_rt::export!(thiscall, rw_00E4CA30(this: u32, arg: u32) -> u32 {
    unsafe {
        const ARG_NAME: u32 = 0x1f8;
        const ARG_QUALITY: u32 = 0x324;
        const NAME_OFF: u32 = 0x209;
        const CHILD: u32 = 0x1e4;
        const CHILD_QUALITY: u32 = 0x224;
        const QUALITY: u32 = 0x430;
        const RECV_NAME: u32 = 0x1ec;
        const RECV_LABEL: u32 = 0x1f4;
        const VSLOT_PUBLISH: u32 = 0x1e0;
        const SUFFIXES: [(u32, u32, u32); 3] =
            [(0x43c, 0x438, 3), (0x444, 0x440, 4), (0x44c, 0x448, 5)];
        const EMPTY_STR: u32 = 0x00fc9c85;
        const RP_LOW: u32 = 0x00f183f8;
        const RP_MED: u32 = 0x00f18408;
        const RP_HIGH: u32 = 0x00f18418;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        unsafe fn strlen(p: u32) -> u32 {
            unsafe {
                let mut n = 0u32;
                while rd8(p.wrapping_add(n)) != 0 {
                    n += 1;
                }
                n
            }
        }
        unsafe fn strcpy(d: u32, s: u32) {
            unsafe {
                let mut n = 0u32;
                loop {
                    let b = rd8(s.wrapping_add(n));
                    wr8(d.wrapping_add(n), b);
                    n += 1;
                    if b == 0 {
                        break;
                    }
                }
            }
        }
        unsafe fn publish(recv: u32, s: u32) {
            unsafe {
                let hook: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(recv) + VSLOT_PUBLISH) as usize);
                hook(recv, s, 0);
            }
        }
        unsafe fn suffix_hit(id: u32, pair: u32, s: u32) -> bool {
            unsafe {
                match id {
                    3 => lf_checker_rt::callee_thiscall!(3, u32, pair, s) != 0,
                    4 => lf_checker_rt::callee_thiscall!(4, u32, pair, s) != 0,
                    _ => lf_checker_rt::callee_thiscall!(5, u32, pair, s) != 0,
                }
            }
        }

        let empty = lf_checker_rt::relocated(EMPTY_STR);
        let mut scratch = [0u8; 32];
        strcpy(scratch.as_mut_ptr() as u32, arg.wrapping_add(ARG_NAME));
        let mut keep = 0u32;
        while scratch[keep as usize] != 0 {
            keep += 1;
        }
        if keep.wrapping_sub(4) >= 0x26 {
            rd8(0);
            return 0;
        }
        scratch[keep.wrapping_sub(4) as usize] = 0;
        let dst = this.wrapping_add(NAME_OFF);
        strcpy(dst, scratch.as_ptr() as u32);

        let q = rd32(arg.wrapping_add(ARG_QUALITY));
        wr32(this.wrapping_add(QUALITY), q);
        let child = rd32(this.wrapping_add(CHILD));
        wr32(child.wrapping_add(CHILD_QUALITY), q);
        lf_checker_rt::callee_thiscall!(1, u32, child, arg);

        let mut conv = 0u32;
        let mut len = 0u32;
        let mut raw = 0u32;
        let mut out = [0u32; 2];
        if dst != 0 {
            let n = strlen(dst);
            lf_checker_rt::callee_thiscall!(
                2,
                u32,
                out.as_mut_ptr() as u32,
                dst,
                n
            );
            raw = out[0];
            conv = out[0];
            len = out[1] & 0xffff;
        }
        for (woff, soff, id) in SUFFIXES {
            let strip = rd16(this.wrapping_add(woff));
            let suf = if strip == 0 {
                empty
            } else {
                rd32(this.wrapping_add(soff))
            };
            if suffix_hit(id, out.as_mut_ptr() as u32, suf) && len > strip {
                len -= strip;
                if len != 0 {
                    wr8(conv.wrapping_add(len), 0);
                }
                break;
            }
        }
        if len == 0 {
            conv = empty;
        }
        publish(rd32(this.wrapping_add(RECV_NAME)), conv);

        let cls: u32 = lf_checker_rt::callee_thiscall!(6, u32, this, dst);
        let mut qraw = 0u32;
        let mut qstr = empty;
        let label = match cls {
            0 => Some((lf_checker_rt::relocated(RP_LOW), 12u32)),
            1 => Some((lf_checker_rt::relocated(RP_MED), 12u32)),
            2 => Some((lf_checker_rt::relocated(RP_HIGH), 11u32)),
            _ => None,
        };
        if let Some((s, n)) = label {
            let mut out2 = [0u32; 2];
            lf_checker_rt::callee_thiscall!(7, u32, out2.as_mut_ptr() as u32, s, n);
            qraw = out2[0];
            if out2[1] & 0xffff != 0 {
                qstr = out2[0];
            }
        }
        publish(rd32(this.wrapping_add(RECV_LABEL)), qstr);

        lf_checker_rt::callee_cdecl!(8, u32, qraw);
        let r: u32 = lf_checker_rt::callee_cdecl!(8, u32, raw);
        lf_checker_rt::callee_thiscall!(10, u32, 0);
        r
    }
});
