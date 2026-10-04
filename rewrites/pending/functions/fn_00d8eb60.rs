// original: 0x00d8eb60 audio_record_scan_bounds
//! Scan one record's samples and store the scaled per-axis bounds.
//!
//! Reads up to 15 samples selected by the record header, probes a 3-vector
//! per sample through the sampler callee while tracking per-axis min/max
//! from the wide initial bounds, then scales by 8 and stores the six bounds
//! as words. Returns the last conversion (as the original falls through with
//! it in EAX).
//!
//! The min/max updates use ordered `>` exactly like the original's
//! `comiss; jbe` chains (NaN never updates), and the final conversions use
//! exact CVTTSS2SI semantics.
export!(thiscall, rw_00d8eb60(this: u32, rec: u32) -> u32 {
    unsafe {
        let table = *((this as *const u32).add(0x60 / 4)) as u32;
        let head = *(rec as *const u32);
        let base = *(((rec as *const u32).add(1))) & 0x1FFFF;
        let count = ((head >> 21) & 0xF) as usize;
        let mut mins = [*global::<f32>(0xEEDE3C); 3];
        let mut maxs = [*global::<f32>(0xEEDE40); 3];
        let mut i = 0usize;
        while i < count {
            let sample =
                *(((table.wrapping_add((base as u32).wrapping_add(i as u32).wrapping_mul(2)))
                    as *const u16)) as u32;
            let mut out = [0f32; 3];
            let _: u32 = callee_thiscall!(1, u32, this, sample, out.as_mut_ptr() as u32);
            let mut k = 0usize;
            while k < 3 {
                // `comiss a,b; jbe` skips the update unless a > b ordered;
                // Rust's `>` is false for NaN exactly like the flag test.
                if mins[k] > out[k] {
                    mins[k] = out[k];
                }
                if out[k] > maxs[k] {
                    maxs[k] = out[k];
                }
                k += 1;
            }
            i += 1;
        }
        let scale = *global::<f32>(0xFE8AFC);
        let c0 = cvttss2si(mins[0] * scale);
        *((rec as *mut u16).add(0x10 / 2)) = c0 as u16;
        let c1 = cvttss2si(mins[1] * scale);
        *((rec as *mut u16).add(0x14 / 2)) = c1 as u16;
        let c2 = cvttss2si(mins[2] * scale);
        *((rec as *mut u16).add(0x18 / 2)) = c2 as u16;
        let c3 = cvttss2si(maxs[0] * scale);
        *((rec as *mut u16).add(0x12 / 2)) = c3 as u16;
        let c4 = cvttss2si(maxs[1] * scale);
        *((rec as *mut u16).add(0x16 / 2)) = c4 as u16;
        let c5 = cvttss2si(maxs[2] * scale);
        *((rec as *mut u16).add(0x1A / 2)) = c5 as u16;
        // The original falls through with the last conversion in EAX.
        c5 as u32
    }
});

/// Truncate-toward-zero float-to-int with exact CVTTSS2SI semantics.
///
/// Rust's `as` saturates out-of-range values and maps NaN to 0; the original
/// instruction yields 0x80000000 for NaN, infinities and out-of-range inputs.
/// (Helper shared with the crate; included so this file stands alone.)
#[inline]
fn cvttss2si(x: f32) -> i32 {
    if !x.is_finite() {
        return i32::MIN;
    }
    let t = x.trunc();
    if t >= 2.0f32.powi(31) || t < -2.0f32.powi(31) {
        return i32::MIN;
    }
    // `t` is finite and in range, so the cast is exact.
    t as i32
}
