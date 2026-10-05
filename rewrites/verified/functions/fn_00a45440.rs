// original: 0x00a45440 vehicle_find_subpart
/// Find a 0x14-byte record by head word, else optionally create it.
///
/// Scans `count` records (`this+0xFA8`) starting at `this+0xFB8` for head
/// word `want` (thiscall, two stack words). On a hit at index `i` returns
/// `this + (5*i+0x3EB)*4`. On a miss (or a zero count) returns 0 when the
/// low byte of `flag` is zero, else calls the creator (callee 1, stdcall,
/// one stack word) with `want` and returns its answer.
export!(thiscall, rw_00a45440(this: u32, want: u32, flag: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0xfa8;
        const RECS_OFF: u32 = 0xfb8;
        const STRIDE: u32 = 0x14;
        const SCALE_ADD: u32 = 0x3eb;
        let count = (this.wrapping_add(COUNT_OFF) as *const u32).read_unaligned();
        let mut p = this.wrapping_add(RECS_OFF);
        let mut i = 0u32;
        let mut found = false;
        while i < count {
            if (p as *const u32).read_unaligned() == want {
                found = true;
                break;
            }
            p = p.wrapping_add(STRIDE);
            i += 1;
        }
        if found {
            return this.wrapping_add(i.wrapping_mul(5).wrapping_add(SCALE_ADD).wrapping_mul(4));
        }
        if (flag & 0xff) == 0 {
            return 0;
        }
        callee_thiscall!(1, u32, this, want)
    }
});
