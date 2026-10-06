// original: 0x00b29ae0 slot_nearest_record

/// Finds the record nearest to a point and remembers its index.
///
/// `slot` selects one of the file slots; `pt` points to three floats
/// (x, y, z). When the slot's record count (compared signed) is not
/// positive, returns 0 without touching the best-index table. Otherwise
/// walks the slot's records (32 bytes apart, the three coordinates 20 bytes
/// into each record) computing squared distance in the original's order
/// ((dx*dx + dy*dy) + dz*dz), square root, and keeps the strictly smallest
/// distance seen starting from 999999.875, recording its byte offset in the
/// best-index table. A NaN distance never wins (the original's branch exits
/// on unordered compare too). Returns the first multiple of 32 at or past
/// the count. Cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_00b29ae0(slot: u32, pt: u32) -> u32 {
    unsafe {
        const SLOT_BASES: u32 = 0x016576B0;
        const SLOT_BEST: u32 = 0x01657710;
        const SLOT_COUNTS: u32 = 0x01657770;
        const REC_STRIDE: u32 = 0x20;
        const COORD_OFF: u32 = 0x14;
        const BEST_INIT: f32 = f32::from_bits(0x497423FE); // 999999.875
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        let count = ((lf_checker_rt::relocated(SLOT_COUNTS) + slot.wrapping_mul(4)) as *const i32)
            .read_unaligned();
        if count <= 0 {
            return 0;
        }
        let base = ((lf_checker_rt::relocated(SLOT_BASES) + slot.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let best_slot =
            (lf_checker_rt::relocated(SLOT_BEST) + slot.wrapping_mul(4)) as *mut u32;
        let px = rdf(pt);
        let py = rdf(pt + 4);
        let pz = rdf(pt + 8);
        let mut best = BEST_INIT;
        let mut rec = base.wrapping_add(COORD_OFF);
        let mut off = 0u32;
        loop {
            let dx = sub(px, rdf(rec));
            let dy = sub(py, rdf(rec + 4));
            let dz = sub(pz, rdf(rec + 8));
            let d = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
            let cur = core::hint::black_box(d).sqrt();
            if best > cur {
                best_slot.write_unaligned(off);
                best = cur;
            }
            off = off.wrapping_add(REC_STRIDE);
            rec = rec.wrapping_add(REC_STRIDE);
            if !((off as i32) < count) {
                break;
            }
        }
        off
    }
});
