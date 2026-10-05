// original: 0x00dd98d0 UIBasicClip::vf133

/// Switch the clip's display mode, refreshing the cached per-slot transforms.
///
/// `this` is the clip; the low byte of `mode` is the requested mode (high
/// bytes ignored). The current mode lives at `+MODE_OFF`: when it already
/// equals the requested mode the function only stores the byte back and makes
/// no calls. Otherwise the mode selects a path:
/// - 0: refresh four slots with zeroed selectors, then notify the part at
///   `+PART` through its virtual slot `NOTIFY_SLOT` with argument 0.
/// - 2: refresh four slots with the `P2_*` selector pairs, no notify.
/// - 1: refresh four slots with zeroed selectors, then refresh two extra
///   slots on the part at `+PART2` (selectors `P1_B_*`), no notify.
/// - any other value: store the mode byte, no calls.
///
/// Each slot refresh calls the part's virtual slot `TRANSFORM_SLOT`
/// (thiscall, no stack arguments) once per path to fetch a handle, resolves
/// it through the matcher (callee 2, thiscall: ECX = two-word out buffer,
/// one stack word = handle) into an entry table of four destination records,
/// then per slot calls the transform service (callee 3, thiscall: ECX =
/// scratch buffer, two stack words = selector pair) and copies the answered
/// 24 bytes to the slot's record at `+COPY_DST`, finishing with the release
/// helper (callee 4, thiscall, ECX = scratch buffer). The matcher's second
/// out word gates a teardown call (callee 6, cdecl, one word = entry table):
/// it runs only when the high half of that word is non-zero. The scratch
/// buffer is never read back. Finally the mode byte is stored at `+MODE_OFF`.
///
/// Original: 0x00dd98d0 (thiscall, one stack word, the callee pops 4 bytes, no return value).
lf_checker_rt::export!(thiscall, rw_00dd98d0(this: u32, mode: u32) -> u32 {
    unsafe {
        const PART: u32 = 0x1e4;
        const PART2: u32 = 0x1e8;
        const MODE_OFF: u32 = 0x2f8;
        const TRANSFORM_SLOT: u32 = 0xf8;
        const NOTIFY_SLOT: u32 = 0x120;
        const COPY_DST: u32 = 0x10;
        const MATCH_CALLEE: u32 = 2;
        const SVC_CALLEE: u32 = 3;
        const RELEASE_CALLEE: u32 = 4;
        const TEARDOWN_CALLEE: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn vcall0(child: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(child) + slot) as usize);
                f(child)
            }
        }
        #[inline(always)]
        unsafe fn copy24(dst: u32, src: u32) {
            unsafe {
                let mut i = 0u32;
                while i < 6 {
                    let v = rd32(src + i * 4);
                    (dst as *mut u32).add(i as usize).write_unaligned(v);
                    i += 1;
                }
            }
        }
        #[inline(always)]
        unsafe fn refresh(buf: u32, entry: u32, sel0: u32, sel1: u32) {
            unsafe {
                let src: u32 = lf_checker_rt::callee_thiscall!(
                    SVC_CALLEE, u32, buf, sel0, sel1);
                copy24(rd32(entry) + COPY_DST, src);
                lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, buf);
            }
        }

        let bl = mode as u8;
        if rd8(this + MODE_OFF) == bl {
            ((this + MODE_OFF) as *mut u8).write(bl);
            return 0;
        }
        let mut scratch = [0u32; 8];
        let buf = scratch.as_mut_ptr() as u32;
        if bl == 0 || bl == 2 || bl == 1 {
            let part = rd32(this + PART);
            let handle = vcall0(part, TRANSFORM_SLOT);
            let mut mbuf = [0u32; 2];
            let mptr = mbuf.as_mut_ptr() as u32;
            lf_checker_rt::callee_thiscall!(MATCH_CALLEE, u32, mptr, handle);
            let table = mbuf[0];
            if bl == 1 {
                refresh(buf, table, 0, 0);
                refresh(buf, table + 4, 0, 0);
                refresh(buf, table + 8, 0, 0);
                refresh(buf, table + 12, 0, 0);
                let part2 = rd32(this + PART2);
                let e1 = rd32(vcall0(part2, TRANSFORM_SLOT));
                let s4: u32 = lf_checker_rt::callee_thiscall!(
                    SVC_CALLEE, u32, buf, 0, 0x40e00000);
                copy24(rd32(e1) + COPY_DST, s4);
                lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, buf);
                let e2 = rd32(vcall0(part2, TRANSFORM_SLOT));
                let s5: u32 = lf_checker_rt::callee_thiscall!(
                    SVC_CALLEE, u32, buf, 0, 0x41400000);
                copy24(rd32(e2 + 4) + COPY_DST, s5);
                lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, buf);
            } else if bl == 0 {
                refresh(buf, table, 0, 0);
                refresh(buf, table + 4, 0, 0);
                refresh(buf, table + 8, 0, 0);
                refresh(buf, table + 12, 0, 0);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(part) + NOTIFY_SLOT) as usize);
                f(part, 0);
            } else {
                refresh(buf, table, 0, 0xc1400000);
                refresh(buf, table + 4, 0x41400000, 0);
                refresh(buf, table + 8, 0, 0);
                refresh(buf, table + 12, 0xc1400000, 0);
            }
            if mbuf[1] & 0xffff0000 != 0 {
                lf_checker_rt::callee_cdecl!(TEARDOWN_CALLEE, u32, table);
            }
        }
        ((this + MODE_OFF) as *mut u8).write(bl);
        0
    }
});
