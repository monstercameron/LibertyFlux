// original: 0x00B35450 partition_28B_key0_asc (proposed)

/// Partition a run of 28-byte records about a pivot key, smallest first.
///
/// Same layout as rw_00b35320 except the key is the float at record offset 0
/// (pivot word 0) and the order is ascending: the low cursor advances while a
/// record's key is below the pivot, the high cursor retreats while a record's
/// key is above it, stopping (NaN included) and exchanging as there.
///
/// Original: 0x00B35450 (cdecl, ten stack words), no callees, no globals.
lf_checker_rt::export!(cdecl, rw_00b35450(first: u32, last: u32, piv_key: u32, _p1: u32, _p2: u32, _p3: u32, _p4: u32, _p5: u32, _p6: u32, _extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 28;
        const KEY_OFF: u32 = 0;

        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Read a record's key, touching the scan's bytes, as the original.
        #[inline(always)]
        unsafe fn scan_key(elem: u32) -> f32 {
            unsafe {
                let a = rd64(elem);
                let b = rd64(elem.wrapping_add(8));
                let c = rd32(elem.wrapping_add(16));
                core::hint::black_box((a, b, c));
                f32::from_bits(rd32(elem.wrapping_add(KEY_OFF)))
            }
        }
        /// Exchange two records (all seven words).
        #[inline(always)]
        unsafe fn swap(lo: u32, hi: u32) {
            unsafe {
                let a0 = rd64(lo);
                let a1 = rd64(lo.wrapping_add(8));
                let a2 = rd64(lo.wrapping_add(16));
                let a3 = rd32(lo.wrapping_add(24));
                let b0 = rd64(hi);
                let b1 = rd64(hi.wrapping_add(8));
                let b2 = rd64(hi.wrapping_add(16));
                let b3 = rd32(hi.wrapping_add(24));
                wr64(lo, b0);
                wr64(lo.wrapping_add(8), b1);
                wr64(lo.wrapping_add(16), b2);
                wr32(lo.wrapping_add(24), b3);
                wr64(hi, a0);
                wr64(hi.wrapping_add(8), a1);
                wr64(hi.wrapping_add(16), a2);
                wr32(hi.wrapping_add(24), a3);
            }
        }

        let pivot = f32::from_bits(piv_key);
        let mut lo = first;
        let mut hi = last;
        loop {
            loop {
                if !(pivot > scan_key(lo)) {
                    break;
                }
                lo = lo.wrapping_add(STRIDE);
            }
            loop {
                hi = hi.wrapping_sub(STRIDE);
                if !(scan_key(hi) > pivot) {
                    break;
                }
            }
            if lo >= hi {
                return lo;
            }
            swap(lo, hi);
            lo = lo.wrapping_add(STRIDE);
        }
    }
});
