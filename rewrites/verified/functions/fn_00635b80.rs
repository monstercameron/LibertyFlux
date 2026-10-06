// original: 0x00635b80 rage::pgDictionary<rage::crAnimation>::vf2 (symbols)

/// Look up the key `[this + KEY]` in the range table `table` and rebase it.
///
/// `table` holds a row count at `+COUNT` (a SIGNED 32-bit value: zero or
/// negative selects nothing) and three parallel arrays of that many words:
/// lower bounds at `+LO`, upper bounds at `+HI` and rebasing addends at
/// `+ADD`. Rows are scanned in order for the first row whose half-open
/// interval contains the key, both bounds compared UNSIGNED
/// (`key < lo` skips the row, `key < hi` takes it). On a hit the key is set
/// to `addend + key` and returned; on a miss the key is untouched and the
/// row count is returned. A non-positive count returns 0 without reading
/// any row. (The original also bails when the hit row index is -1, which
/// the scan cannot produce.)
///
/// Returns the rebased key on a hit, the row count on a miss, 0 when the
/// count is not positive.
///
/// Original: 0x00635b80 (thiscall, one stack word, no calls, no globals).
lf_checker_rt::export!(thiscall, rw_00635b80(this: u32, table: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x08;
        const LO: u32 = 0x000;
        const HI: u32 = 0x200;
        const ADD: u32 = 0x400;
        const COUNT: u32 = 0x600;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let key = rd32(this + KEY);
        let count = rd32(table + COUNT) as i32;
        if count <= 0 {
            return 0;
        }
        let mut i = 0u32;
        while (i as i32) < count {
            let lo = rd32(table + LO + i.wrapping_mul(4));
            if key < lo {
                i = i.wrapping_add(1);
                continue;
            }
            let hi = rd32(table + HI + i.wrapping_mul(4));
            if key < hi {
                let v = rd32(table + ADD + i.wrapping_mul(4)).wrapping_add(key);
                wr32(this + KEY, v);
                return v;
            }
            i = i.wrapping_add(1);
        }
        i
    }
});
