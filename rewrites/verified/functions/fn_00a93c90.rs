// original: 0x00a93c90 stream_notify_range (proposed)

/// Notify the handler of every live id in range `idx`.
///
/// The range table global holds `(lo, hi)` pairs; the id table global holds
/// the ids. For `k` in `lo..hi` (signed, empty when `lo >= hi`), ids equal
/// to -1 are skipped and the rest are reported to the callee with the
/// device handle global and the constant `0x40`. Returns what the last
/// iteration left in `eax`: the callee's answer after a report, -1 after a
/// skip, or the range table base when the range was empty.
///
/// Original: stdcall, one stack argument. One callee (cdecl, 3 args).
lf_checker_rt::export!(stdcall, rw_00a93c90(idx: u32) -> u32 {
    unsafe {
        const RANGE_TABLE_G: u32 = 0x012fb378;
        const ID_TABLE_G: u32 = 0x012fb388;
        const HANDLE_G: u32 = 0x012b4138;
        const SKIP_ID: u32 = 0xffff_ffff;
        const NOTIFY_FLAGS: u32 = 0x40;
        const NOTIFY: u32 = 0;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let ranges = rd32(lf_checker_rt::relocated(RANGE_TABLE_G));
        let ids = rd32(lf_checker_rt::relocated(ID_TABLE_G));
        let handle = rd32(lf_checker_rt::relocated(HANDLE_G));
        let mut k = rd32(ranges.wrapping_add(idx.wrapping_mul(4))) as i32;
        let hi = rd32(ranges.wrapping_add(idx.wrapping_mul(4)).wrapping_add(4)) as i32;
        let mut last = ranges;
        while k < hi {
            let v = rd32(ids.wrapping_add((k as u32).wrapping_mul(4)));
            if v != SKIP_ID {
                last = lf_checker_rt::callee_cdecl!(NOTIFY, u32, v, handle, NOTIFY_FLAGS);
            } else {
                last = v;
            }
            k = k.wrapping_add(1);
        }
        last
    }
});
