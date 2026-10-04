// original: 0x00E4CC60 video_export_apply_quality (proposed)

/// Store a clip name on the export panel object, strip a known container
/// suffix from its converted form, publish both strings through the panel's
/// child widgets, and release the converted buffers.
///
/// `this` is the export panel object, `name` a NUL-terminated byte string.
/// The name is copied to the panel's name field (`+0x209`), the child object
/// at `+0x1e4` is told about it through callee 1, and the panel's quality
/// value (`+0x430`) is mirrored to the child's `+0x224` slot. Callee 2
/// converts the stored name into a (pointer, length) pair.
///
/// The converted string is tested against three (length, string) pairs on the
/// panel (`+0x43c`/`+0x438`, `+0x444`/`+0x440`, `+0x44c`/`+0x448`; a zero
/// length selects a shared empty string). The first pair whose string matches
/// the converted tail, with the converted length above the pair's length,
/// strips that many trailing characters and re-terminates the string. An
/// empty result (or an empty conversion) selects the shared empty string.
///
/// The surviving string is published through virtual slot `+0x1e0` of the
/// object at `+0x1ec` (two stack words: string, 0). Callee 3 then classifies
/// the stored name; its answer 0, 1 or 2 selects one of three quality labels
/// ("RP_EXPORTLOW", "RP_EXPORTMED", "RP_EXPORTHI", with lengths 12, 12, 11)
/// which callee 4 converts the same way (an empty conversion, or any other
/// answer, selects the shared empty string), and the result is published
/// through slot `+0x1e0` of the object at `+0x1f4`. Both converted buffers
/// are released through callee 5 (cdecl, one pointer), quality buffer first.
///
/// Returns callee 5's answer for the name buffer. Original: 0x00E4CC60
/// (thiscall, one stack word). Callee ids: 1 child sync, 2 convert (name),
/// 3 classify, 4 convert (quality label), 5 release, 6 virtual slot +0x1e0.
lf_checker_rt::export!(thiscall, rw_00E4CC60(this: u32, name: u32) -> u32 {
    unsafe {
        const NAME_OFF: u32 = 0x209;
        const CHILD: u32 = 0x1e4;
        const CHILD_QUALITY: u32 = 0x224;
        const QUALITY: u32 = 0x430;
        const RECV_NAME: u32 = 0x1ec;
        const RECV_LABEL: u32 = 0x1f4;
        const VSLOT_PUBLISH: u32 = 0x1e0;
        const SUFFIXES: [(u32, u32); 3] = [(0x43c, 0x438), (0x444, 0x440), (0x44c, 0x448)];
        const EMPTY_STR: u32 = 0x00fc9c85;
        const RP_LOW: u32 = 0x00f183cc;
        const RP_MED: u32 = 0x00f183dc;
        const RP_HIGH: u32 = 0x00f183ec;

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
        /// True when the last `slen` bytes at (buf, buflen) equal `suf`.
        /// Compared from the end like the original.
        unsafe fn tail_matches(buf: u32, buflen: u32, suf: u32, slen: u32) -> bool {
            unsafe {
                let mut k = slen;
                while k > 0 {
                    k -= 1;
                    if rd8(buf.wrapping_add(buflen.wrapping_sub(slen).wrapping_add(k)))
                        != rd8(suf.wrapping_add(k))
                    {
                        return false;
                    }
                }
                true
            }
        }
        unsafe fn publish(recv: u32, s: u32) {
            unsafe {
                let hook: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(recv) + VSLOT_PUBLISH) as usize);
                hook(recv, s, 0);
            }
        }

        let empty = lf_checker_rt::relocated(EMPTY_STR);
        let dst = this.wrapping_add(NAME_OFF);
        strcpy(dst, name);
        let child = rd32(this.wrapping_add(CHILD));
        lf_checker_rt::callee_thiscall!(1, u32, child, name);
        wr32(child.wrapping_add(CHILD_QUALITY), rd32(this.wrapping_add(QUALITY)));

        let mut conv = 0u32;
        let mut len = 0u32;
        let mut raw = 0u32;
        if dst != 0 {
            let n = strlen(dst);
            let mut out = [0u32; 2];
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
        for (woff, soff) in SUFFIXES {
            let strip = rd16(this.wrapping_add(woff));
            let suf = if strip == 0 {
                empty
            } else {
                rd32(this.wrapping_add(soff))
            };
            let slen = strlen(suf);
            if slen <= len && tail_matches(conv, len, suf, slen) && len > strip {
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

        let cls: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, dst);
        let mut qraw = 0u32;
        let mut qstr = empty;
        let label = match cls {
            0 => Some((lf_checker_rt::relocated(RP_LOW), 12u32)),
            1 => Some((lf_checker_rt::relocated(RP_MED), 12u32)),
            2 => Some((lf_checker_rt::relocated(RP_HIGH), 11u32)),
            _ => None,
        };
        if let Some((s, n)) = label {
            let mut out = [0u32; 2];
            lf_checker_rt::callee_thiscall!(4, u32, out.as_mut_ptr() as u32, s, n);
            qraw = out[0];
            if out[1] & 0xffff != 0 {
                qstr = out[0];
            }
        }
        publish(rd32(this.wrapping_add(RECV_LABEL)), qstr);

        lf_checker_rt::callee_cdecl!(5, u32, qraw);
        lf_checker_rt::callee_cdecl!(5, u32, raw)
    }
});
