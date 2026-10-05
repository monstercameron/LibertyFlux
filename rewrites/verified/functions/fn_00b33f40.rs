// original: 0x00B33F40 insertion_sort_28B_key0_asc (proposed)

/// Sort a run of 28-byte records by ascending float key (insertion sort).
///
/// `first`/`last` bound the run (`last` exclusive, a whole number of records
/// past `first`). The third word is unread; `extra` is passed through to the
/// linear-insert callee. Records compare by the float at record offset 0.
///
/// Each record past the first is handled in turn: when the front record's key
/// is at or below the current record's key (NaN included) the current record
/// is handed to the linear-insert callee by value (hole, seven words) together
/// with `extra`; otherwise the front run is shifted up one record by the shift
/// callee (front, current, current + one, scratch address) and the current
/// record is copied to the front. Runs shorter than two records return at
/// once. Returns void (the original leaves garbage in eax).
///
/// Original: 0x00B33F40 (cdecl, four stack words), two callees, no globals.
lf_checker_rt::export!(cdecl, rw_00b33f40(first: u32, last: u32, _dead: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 28;

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
                f32::from_bits(rd32(elem))
            }
        }

        if first == last {
            return 0;
        }
        let mut cur = first.wrapping_add(STRIDE);
        if cur == last {
            return 0;
        }
        loop {
            if !(scan_key(first) > scan_key(cur)) {
                let w0 = rd32(cur);
                let w1 = rd32(cur.wrapping_add(4));
                let w2 = rd32(cur.wrapping_add(8));
                let w3 = rd32(cur.wrapping_add(12));
                let w4 = rd32(cur.wrapping_add(16));
                let w5 = rd32(cur.wrapping_add(20));
                let w6 = rd32(cur.wrapping_add(24));
                lf_checker_rt::callee_cdecl!(2, u32, cur, w0, w1, w2, w3, w4, w5, w6, extra);
            } else {
                // Fourth argument is the original's scratch address; the
                // contract skips it in the call comparison.
                lf_checker_rt::callee_cdecl!(1, u32, first, cur, cur.wrapping_add(STRIDE), 0u32);
                let w0 = rd64(cur);
                let w1 = rd64(cur.wrapping_add(8));
                let w2 = rd64(cur.wrapping_add(16));
                let w3 = rd32(cur.wrapping_add(24));
                wr64(first, w0);
                wr64(first.wrapping_add(8), w1);
                wr64(first.wrapping_add(16), w2);
                wr32(first.wrapping_add(24), w3);
            }
            cur = cur.wrapping_add(STRIDE);
            if cur == last {
                return 0;
            }
        }
    }
});
