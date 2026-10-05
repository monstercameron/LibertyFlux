// original: 0x00D44AB0 peds_task_weighted_pick (proposed)

/// Pick one of up to 16 weighted entries by a random threshold over prefix sums.
///
/// `this` points to the picker object: 16 entry records of 32 bytes at `+0x00`,
/// 16 cached result slots at `+0x200` (`-1` means empty), 16 float weights at
/// `+0x240`, and the entry count at `+0x280`. The count is used as a signed
/// value and the object only holds 16 entries, so honest counts are 0..=16.
///
/// Behaviour: build the prefix sums of the weights (the original sums the
/// first multiple-of-four entries as `weight + running` and the tail as
/// `running + weight`; both orders are replicated for bit-exact floats),
/// draw the random callee, form `threshold = float(rand) * SCALE * total`,
/// and take the first index whose prefix strictly exceeds the threshold, or
/// the last index when none does. A count of zero or less returns `-1`
/// without scanning. The picked slot is returned as-is unless it holds `-1`,
/// in which case the fill callee computes it through the slot pointer and
/// the filled value is returned. Every exit runs the security-cookie check.
///
/// Original: 0x00D44AB0 (thiscall, no stack words, returns u32).
lf_checker_rt::export!(thiscall, rw_00D44AB0(this: u32) -> u32 {
    unsafe {
        const ENTRY_STRIDE: u32 = 32;
        const MAX_ENTRIES: usize = 16;
        const SLOTS: u32 = 0x200;
        const WEIGHTS: u32 = 0x240;
        const COUNT: u32 = 0x280;
        const EMPTY: u32 = 0xFFFF_FFFF;
        const SCALE: f32 = f32::from_bits(0x3800_0100);
        const RAND_CALLEE: u32 = 1;
        const COOKIE_CALLEE: u32 = 2;
        const FILL_CALLEE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe { lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,) };
        }

        let count = rd32(this.wrapping_add(COUNT)) as i32;
        let mut prefix = [0.0f32; MAX_ENTRIES];
        let mut total = 0.0f32;
        if count > 0 {
            let n = (count as u32).min(MAX_ENTRIES as u32);
            // The original sums whole groups of four first (only when the
            // count reaches four), then the tail one by one.
            let nvec = if count >= 4 { n & !3 } else { 0 };
            let mut i = 0u32;
            while i < nvec {
                let p = add(rdf(this.wrapping_add(WEIGHTS).wrapping_add(i * 4)), total);
                prefix[i as usize] = p;
                total = p;
                i += 1;
            }
            while i < n {
                total = add(total, rdf(this.wrapping_add(WEIGHTS).wrapping_add(i * 4)));
                prefix[i as usize] = total;
                i += 1;
            }
        }
        let rand = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,);
        // The original converts with cvtdq2ps: signed int to float.
        let threshold = mul(mul((rand as i32) as f32, SCALE), total);
        if count <= 0 {
            cookie();
            return EMPTY;
        }
        let n = (count as u32).min(MAX_ENTRIES as u32);
        let mut idx = 0u32;
        while idx < n {
            if prefix[idx as usize] > threshold {
                break;
            }
            if idx + 1 >= n {
                break;
            }
            idx += 1;
        }
        let slot = this.wrapping_add(SLOTS).wrapping_add(idx * 4);
        if rd32(slot) == EMPTY {
            let obj = this.wrapping_add(idx * ENTRY_STRIDE);
            lf_checker_rt::callee_cdecl!(FILL_CALLEE, u32, obj, slot);
        }
        let out = rd32(slot);
        cookie();
        out
    }
});
