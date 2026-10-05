// original: 0x00a8ace0 pool_slot_valid_check (proposed)

/// Check whether a value names a live slot of this pool, returning 1 or 0.
///
/// `this` describes the pool: base at +0, flag table at +4, entry count at
/// +8 and stride at +0xC. `val` must lie in [base, base+(count-1)*stride]
/// (unsigned), must be aligned to the stride, and the flag byte at
/// table+(val-base)/stride must not have bit 0x80 set. Returns 1 in the low
/// byte on success and 0 in the low byte on failure, leaving the other
/// bytes as the original's arithmetic leaves them. The proof pins the
/// stride to 4 and the count to 8 (a zero stride faults the original with
/// a divide error the rewrite cannot reproduce).
///
/// Original: 0x00A8ACE0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8ace0(this: u32, val: u32) -> u32 {
    unsafe {
        const BASE: u32 = 0x0;
        const FLAGS: u32 = 0x4;
        const COUNT: u32 = 0x8;
        const STRIDE: u32 = 0xc;
        const DEAD_BIT: u8 = 0x80;
        let base = ((this + BASE) as *const u32).read_unaligned();
        if val < base {
            return val & 0xffffff00;
        }
        let count = ((this + COUNT) as *const u32).read_unaligned();
        let stride = ((this + STRIDE) as *const u32).read_unaligned();
        let end = base.wrapping_add(
            (count.wrapping_sub(1)).wrapping_mul(stride),
        );
        if val > end {
            return val & 0xffffff00;
        }
        let index = (val.wrapping_sub(base) as i32) / (stride as i32);
        let rem = (val.wrapping_sub(base) as i32) % (stride as i32);
        if rem != 0 {
            return (index as u32) & 0xffffff00;
        }
        let table = ((this + FLAGS) as *const u32).read_unaligned();
        let flag =
            ((table.wrapping_add(index as u32)) as *const u8).read();
        if flag & DEAD_BIT != 0 {
            return (index as u32) & 0xffffff00;
        }
        ((index as u32) & 0xffffff00) | 1
    }
});
