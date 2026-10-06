// original: 0x00963820 access_check_plain
/// Decide whether a keyed access is allowed (plain form).
///
/// Arguments are `(key, mask, flags, lo, hi)`. Rejects (returns 0) when
/// `flags` is negative (signed), the enable byte at `0x10376E8` is clear,
/// or the lookup helper (cdecl/1) answer shares no bit with `mask`. Any
/// answer other than `0x100` accepts; `0x100` additionally needs mode `1`
/// at `0x1037720` and `lo >= hi` (unsigned). The result is the low byte;
/// upper bits pass through the helper answer, except on the two range
/// exits, which carry `lo`'s upper bits (EAX is reloaded first), and the
/// first two exits, which pass the caller's EAX (so the proof pins entry
/// EAX below 256); all are reproduced exactly.
lf_checker_rt::export!(cdecl, rw_00963820(key: u32, mask: u32, flags: u32, lo: u32, hi: u32) -> u32 {
    unsafe {
        const ENABLED: u32 = 0x10376e8;
        const MODE: u32 = 0x1037720;
        const NEEDS_RANGE: u32 = 0x100;
        if (flags as i32) < 0 {
            return 0;
        }
        if (lf_checker_rt::global::<u8>(ENABLED) as *const u8).read() == 0 {
            return 0;
        }
        let r: u32 = lf_checker_rt::callee_cdecl!(1, u32, key);
        if (mask & r) == 0 {
            return r & 0xffff_ff00;
        }
        if r != NEEDS_RANGE {
            return (r & 0xffff_ff00) | 1;
        }
        if (lf_checker_rt::global::<u32>(MODE) as *const u32).read_unaligned() != 1 {
            return r & 0xffff_ff00;
        }
        // The original reloads EAX with `lo` before the unsigned compare,
        // so both range exits carry `lo`'s upper bits, not the answer's.
        if lo < hi {
            return lo & 0xffff_ff00;
        }
        (lo & 0xffff_ff00) | 1
    }
});
