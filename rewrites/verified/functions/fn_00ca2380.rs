// original: 0x00ca2380 slot_find

/// Search the five event-source slots for a value.
///
/// Scans the five words at `+0x15c` for `val`. The low byte of the return
/// value is 1 on a match and 0 otherwise. The upper three bytes are the
/// scanning pointer with its low byte replaced: the matching slot's address
/// on success, the address one past the last slot on failure. Callers use
/// only the low byte, but the full value is deterministic and reproduced.
///
/// Original: 0x00ca2380 (thiscall, one stack word, returns al).
lf_checker_rt::export!(thiscall, rw_00ca2380(this: u32, val: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe {
        const SLOTS: u32 = 0x15c;
        const COUNT: u32 = 5;
        let mut i = 0u32;
        while i < COUNT {
            let p = this.wrapping_add(SLOTS).wrapping_add(i * 4);
            if rd32(p) == val {
                return (p & 0xffff_ff00) | 1;
            }
            i += 1;
        }
        this.wrapping_add(SLOTS).wrapping_add(COUNT * 4) & 0xffff_ff00
    }
});
