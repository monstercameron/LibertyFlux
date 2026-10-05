// original: 0x009389F0 stream_cache_refresh_b (proposed)

/// Re-stamp the streaming marker when the count has grown past it.
///
/// Needs a quiet poll, a clear flag, a nonzero mode and a nonempty scan.
/// When the marker already covers the counter it is re-stamped and 0 is
/// answered. Otherwise the handle is opened: a null handle, a matching
/// stamp word, or (unreachable: the mode is already known nonzero) a zero
/// mode reaches the tail, which answers 1 except for state 0 with mode 1.
/// A mismatching stamp with nonzero mode flushes and answers 0.
lf_checker_rt::export!(cdecl, rw_009389f0() -> u32 {
    unsafe {
        const POLL: u32 = 1;
        const SCAN: u32 = 2;
        const COUNT: u32 = 3;
        const OPEN: u32 = 4;
        const FLUSH: u32 = 5;
        const FLAG: u32 = 0x11609F6;
        const MODE: u32 = 0x11A4EF4;
        const MARKER: u32 = 0x11A4EFC;
        const STAMP: u32 = 0x1284644;
        const WANT: u32 = 0x11A4F10;
        const STATE: u32 = 0x11A4EF8;
        const TAG_OFF: u32 = 0x2C;
        const LOW_MASK: u32 = 0xFFFF_FF00;
        let a: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
        if (a & 0xFF) != 0 {
            return a & LOW_MASK;
        }
        if lf_checker_rt::global::<u8>(FLAG).read() != 0 {
            return a & LOW_MASK;
        }
        if lf_checker_rt::global::<u32>(MODE).read_unaligned() == 0 {
            return a & LOW_MASK;
        }
        let s: u32 = lf_checker_rt::callee_cdecl!(SCAN, u32,);
        if (s & 0xFF) == 0 {
            return s & LOW_MASK;
        }
        let e: u32 = lf_checker_rt::callee_cdecl!(COUNT, u32,);
        let marker = lf_checker_rt::global::<u32>(MARKER);
        if marker.read_unaligned() >= e {
            marker.write_unaligned(lf_checker_rt::global::<u32>(STAMP).read_unaligned());
            return e & LOW_MASK;
        }
        let p: u32 = lf_checker_rt::callee_cdecl!(OPEN, u32, 0);
        let m = lf_checker_rt::global::<u32>(MODE).read_unaligned();
        if p != 0 {
            let want = lf_checker_rt::global::<u32>(WANT).read_unaligned() & 0xFFFF;
            let have = ((p + TAG_OFF) as *const u16).read_unaligned() as u32;
            if want != have && m != 0 {
                let r: u32 = lf_checker_rt::callee_cdecl!(FLUSH, u32, 1);
                return r & LOW_MASK;
            }
        }
        // Tail: state 3, 4 or anything else nonzero answers 1, as does
        // state 0 with any mode but 1; state 0 with mode 1 flushes.
        let v = lf_checker_rt::global::<u32>(STATE).read_unaligned();
        if v != 0 || m != 1 {
            return (p & LOW_MASK) | 1;
        }
        let r: u32 = lf_checker_rt::callee_cdecl!(FLUSH, u32, 1);
        r & LOW_MASK
    }
});
