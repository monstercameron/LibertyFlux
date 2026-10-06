// original: 0x00963870 access_check_offset
/// Decide whether a keyed access is allowed (offset form).
///
/// Same predicate as `access_check_plain` with two more arguments
/// `(key, mask, flags, lo, hi, _unused, base)`: on the accepting `0x100`
/// path the upper return bits carry `base + hi` instead of the helper
/// answer. Result and pass-through rules are otherwise identical.
lf_checker_rt::export!(cdecl, rw_00963870(key: u32, mask: u32, flags: u32, lo: u32, hi: u32, _unused: u32, base: u32) -> u32 {
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
        if lo < hi {
            return r & 0xffff_ff00;
        }
        let s = base.wrapping_add(hi);
        (s & 0xffff_ff00) | 1
    }
});
