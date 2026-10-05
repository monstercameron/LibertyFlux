// original: 0x00d15950 gather_filtered_position_average (proposed)
//
// Scans a 16-slot table for entries that pass two filters, averages the
// positions of the ones within range, and reports the count.
//
// `a1` is an actor whose +0x224 word points at a record holding the slot
// array at +0x168 (16 object pointers, nulls skipped); `a2` is one favoured
// entry; `a3` points at the reference position (three floats); `a4` is a
// distance limit as float bits; `a5` takes four output floats (average x/y/z
// plus a slot the original fills from an uninitialized stack word, always
// zero under the contract's stack fill); `a6` takes the entry count; only
// the low byte of `a7` is read. Returns nonzero on success, zero otherwise.
//
// Each non-null entry `e` (position at [e+0x20]+0x30, three floats) is
// tested: squared distance to the reference is formed in the order
// (dy*dy + dx*dx) + dz*dz. Filter 1 (callee 1, thiscall on the +0x224
// record with `e`) passing counts as a pass on its own; otherwise the entry
// passes when it is `a2` or filter 2 (callee 2, same record, `e` and 1)
// passes. The entry itself is skipped when it is `a1`. A passing entry
// whose distance exceeds the squared radius (6.0 from writable data, read
// through the checker's global) updates nothing; otherwise the overall best
// distance starts at 99999.0 and keeps the minimum, a second best (same
// start) keeps the minimum over entries that passed with the second filter
// active, and the position joins running sums with the count bumped.
//
// After the scan, an empty set fails. Otherwise the overall best must not
// exceed the squared limit `a4`, else the low byte of `a7` must be clear
// and the second best must not exceed 9.0 otherwise. On success the count
// is stored to `a6` when non-null, and when `a5` is non-null it receives
// (1.0/count)*sum per axis in that operand order, plus the zero word.
//
// The original returns leftovers in the upper bytes of eax: the fail paths
// return the slot-array base with the low byte cleared (every loop
// iteration ends by reloading the base, or preserves it on null slots), and
// the success path returns 1, or the `a5` pointer with its low byte set.
// Original convention: cdecl, seven stack words.
lf_checker_rt::export!(cdecl, rw_00d15950(
    a1: u32,
    a2: u32,
    a3: u32,
    a4bits: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    unsafe {
        const REC: u32 = 0x224;
        const SLOTS: u32 = 0x168;
        const NSLOT: u32 = 16;
        const POS: u32 = 0x20;
        const RADIUS_FILE_VA: u32 = 0x1053cc4;
        const BEST_INIT: f32 = f32::from_bits(0x47c34f80); // 99999.0
        const SECOND_LIMIT: f32 = f32::from_bits(0x41100000); // 9.0
        const SCALE_ONE: f32 = f32::from_bits(0x3f800000); // 1.0

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// `comiss a, b` + `jbe`: taken unless a is above b.
        #[inline(always)]
        fn below_eq(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) > core::hint::black_box(b))
        }
        /// `comiss a, b` + `ja`: taken when a is strictly above b.
        #[inline(always)]
        fn above(a: f32, b: f32) -> bool {
            core::hint::black_box(a) > core::hint::black_box(b)
        }

        let rec = rd32(a1 + REC);
        let base = rec.wrapping_add(SLOTS);
        let radius: f32 = unsafe { *lf_checker_rt::global::<f32>(RADIUS_FILE_VA) };
        let radius2 = mul(radius, radius);
        let mut best = BEST_INIT;
        let mut second = BEST_INIT;
        let mut sum0 = 0.0f32;
        let mut sum1 = 0.0f32;
        let mut sum2 = 0.0f32;
        let mut count = 0u32;
        let mut i = 0u32;
        while i < NSLOT {
            let e = rd32(base + i * 4);
            if e != 0 {
                let p = rd32(e + POS);
                let dx = sub(rdf(p + 0x30), rdf(a3));
                let dy = sub(rdf(p + 0x34), rdf(a3 + 4));
                let dz = sub(rdf(p + 0x38), rdf(a3 + 8));
                let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                let c1: u32 = lf_checker_rt::callee_thiscall!(1, u32, rec, e);
                let cl = (c1 as u8) != 0;
                let al = if cl {
                    false
                } else if e == a2 {
                    true
                } else {
                    let c2: u32 = lf_checker_rt::callee_thiscall!(2, u32, rec, e, 1);
                    (c2 as u8) != 0
                };
                if e != a1 && (cl || al) {
                    if !below_eq(radius2, dist2) {
                        if !below_eq(best, dist2) {
                            best = dist2;
                        }
                        if al {
                            if !below_eq(second, dist2) {
                                second = dist2;
                            }
                        }
                        count += 1;
                        sum0 = add(rdf(p + 0x30), sum0);
                        sum1 = add(rdf(p + 0x34), sum1);
                        sum2 = add(rdf(p + 0x38), sum2);
                    }
                }
            }
            i += 1;
        }
        if count == 0 {
            return base & 0xffffff00;
        }
        let limit = f32::from_bits(a4bits);
        let limit2 = mul(limit, limit);
        if above(best, limit2) {
            if (a7 as u8) != 0 {
                return base & 0xffffff00;
            }
            if above(second, SECOND_LIMIT) {
                return base & 0xffffff00;
            }
        }
        if a6 != 0 {
            wr32(a6, count);
        }
        if a5 != 0 {
            let x = div(SCALE_ONE, count as f32);
            wrf(a5, mul(x, sum0));
            wrf(a5 + 4, mul(x, sum1));
            wrf(a5 + 8, mul(x, sum2));
            // The original copies an uninitialized below-ESP word here; the
            // contract fills uninitialized stack with zero.
            wr32(a5 + 0xc, 0);
            return (a5 & 0xffffff00) | 1;
        }
        1
    }
});
