// original: 0x009389A0 stream_cache_refresh_a (proposed)

/// Refresh the streaming cache marker when the count has grown.
///
/// Needs the active flag set and a nonzero mode. An empty scan first
/// drains the queue. When the stored marker is below the counter the
/// marker is left for later and 1 is answered; otherwise the fresh stamp
/// is stored, the queue is flushed and 0 is answered. The upper bits of
/// EAX are always the last call's leftover, reproduced exactly.
lf_checker_rt::export!(cdecl, rw_009389a0() -> u32 {
    unsafe {
        const POLL: u32 = 1;
        const SCAN: u32 = 2;
        const DRAIN: u32 = 3;
        const COUNT: u32 = 4;
        const FLUSH: u32 = 5;
        const ACTIVE: u32 = 0x11609F6;
        const MODE: u32 = 0x11A4EF4;
        const MARKER: u32 = 0x11A4EFC;
        const STAMP: u32 = 0x1284644;
        const LOW_MASK: u32 = 0xFFFF_FF00;
        let first: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
        if lf_checker_rt::global::<u8>(ACTIVE).read() == 0 {
            return first & LOW_MASK;
        }
        if lf_checker_rt::global::<u32>(MODE).read_unaligned() == 0 {
            return first & LOW_MASK;
        }
        let s: u32 = lf_checker_rt::callee_cdecl!(SCAN, u32,);
        if (s & 0xFF) == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(DRAIN, u32,);
        }
        let e: u32 = lf_checker_rt::callee_cdecl!(COUNT, u32,);
        let marker = lf_checker_rt::global::<u32>(MARKER);
        if marker.read_unaligned() < e {
            return (e & LOW_MASK) | 1;
        }
        marker.write_unaligned(lf_checker_rt::global::<u32>(STAMP).read_unaligned());
        let r: u32 = lf_checker_rt::callee_cdecl!(FLUSH, u32, 0);
        r & LOW_MASK
    }
});
