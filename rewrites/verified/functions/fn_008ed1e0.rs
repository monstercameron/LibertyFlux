// original: 0x008ed1e0 radius_probe_2d
/// Test whether any flagged node lies within `bound` of a 2D point.
///
/// Scans all 64 groups; only nodes with flag bit 2 take part. Returns
/// `(vec & !0xFF) | 1` on the first node closer than `bound`, else 0.
export!(thiscall, rw_008ed1e0(
    this: *const u8,
    vec: *const f32,
    bound: f32,
) -> u32 {
    unsafe {
        let k: f32 = *global::<f32>(0xFE87A4);
        let mut i = 0u32;
        while i < 64 {
            let base = *(this.add((0x804 + i * 4) as usize) as *const u32);
            let count =
                *(this.add((0xB04 + i * 4) as usize) as *const i32);
            i += 1;
            if base == 0 {
                continue;
            }
            if count <= 0 {
                continue;
            }
            let mut j = 0i32;
            while j < count {
                let node = base
                    .wrapping_add(0x14)
                    .wrapping_add((j as u32).wrapping_mul(0x20));
                j += 1;
                if *((node + 0xb) as *const u8) & 2 == 0 {
                    continue;
                }
                let px = (node as *const i16).read_unaligned() as f32 * k;
                let py = ((node + 2) as *const i16).read_unaligned() as f32
                    * k;
                let dx = *vec - px;
                let dy = *vec.add(1) - py;
                if bound > (dx * dx + dy * dy).sqrt() {
                    return (vec as u32 & !0xFF) | 1;
                }
            }
        }
        0
    }
});
