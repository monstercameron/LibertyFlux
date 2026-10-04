// original: 0x008ead60 y_range_index_scan
/// Find the index range whose node heights bracket `[lo, hi]`.
///
/// Scans upward from the start for the last index with height below `lo`,
/// then downward from the end for the first index with height above `hi`,
/// writing both into `out_lo`/`out_hi`. Unordered (NaN) comparisons count
/// as in-range, matching the original's branchless-compare behaviour.
export!(thiscall, rw_008ead60(
    this: *const u8,
    idx: u32,
    lo: f32,
    hi: f32,
    out_lo: *mut u32,
    out_hi: *mut u32,
) -> () {
    unsafe {
        const STEP: i32 = 0x14;
        const STRIDE: u32 = 32;
        let k: f32 = *global::<f32>(0xFE87A4);
        let count = *(this.add((0xB04 + idx * 4) as usize) as *const i32);
        let base = *(this.add((0x804 + idx * 4) as usize) as *const u32);
        *out_lo = 0;
        *out_hi = count as u32;
        let height = |at: i32| {
            ((base.wrapping_add((at as u32).wrapping_mul(STRIDE)) + 0x16)
                as *const i16)
                .read_unaligned() as f32
                * k
        };
        let mut edx = STEP;
        while edx < *out_hi as i32 {
            if height(edx) >= lo {
                break;
            }
            *out_lo = edx as u32;
            edx += STEP;
        }
        let mut ecx = *out_hi as i32 - STEP;
        while ecx >= *out_lo as i32 {
            if hi >= height(ecx) {
                break;
            }
            *out_hi = ecx as u32;
            ecx -= STEP;
        }
    }
});
