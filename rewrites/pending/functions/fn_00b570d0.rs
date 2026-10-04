// original: 0x00b570d0 normalize_band_levels
// rs05f7: normalise per-band levels against a running budget.
//
// For bands 9 down to 0: sums the levels whose kind matches the band; when
// the sum exceeds the running residual, rescales every matching level (by
// the residual when more than one matched, else stores the residual) and
// zeroes the residual, otherwise subtracts the sum from it. The residual
// starts at a global constant. The original unrolls the scan four-wide; the
// additions happen in the same index order here, so float results are
// bit-identical.
export!(cdecl, rw_b570d0(levels: *mut f32, kinds: *const u32, count: u32) -> () {
    unsafe {
        let mut residual = *global::<f32>(0xFE88E8);
        let mut band = 9i32;
        loop {
            let mut sum = 0f32;
            let mut hits = 0u32;
            for i in 0..count {
                if *kinds.add(i as usize) == band as u32 {
                    sum += *levels.add(i as usize);
                    hits += 1;
                }
            }
            if sum > residual {
                if count > 0 {
                    for i in 0..count {
                        if *kinds.add(i as usize) == band as u32 {
                            if hits > 1 {
                                *levels.add(i as usize) *= residual;
                            } else {
                                *levels.add(i as usize) = residual;
                            }
                        }
                    }
                }
                residual = 0.0;
            } else {
                residual -= sum;
            }
            band -= 1;
            if band < 0 {
                break;
            }
        }
    }
});
