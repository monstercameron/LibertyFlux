// original: 0x00AD5850 audio_range_best
/// Nearest-entry search over two parallel range tables.
///
/// Scans the entry array for the entry whose (x, y) range box is closest to
/// the point (`x`, `y`), where closeness is the Euclidean distance to the
/// box (zero on any axis whose coordinate falls inside that axis' range).
/// Returns the winning distance, or 0.0 when either coordinate is outside
/// the open (-3000, 3000) window (NaN included: every bound check is an
/// ordered comparison, so unordered inputs take the out-of-range exit).
///
/// When the low byte of `flags` equals 1, the winning entry's value is also
/// published to the audio globals: the raw value first, then zeroed again if
/// the combined distance exceeds 70. All range endpoints are 16-bit table
/// values converted to `f32` exactly the way the original converts them.
export!(cdecl, rw_ad5850(x: f32, y: f32, flags: u32) -> f32 {
    const LO: f32 = -3000.0;
    const HI: f32 = 3000.0;
    const BIG: f32 = 10000000.0;
    const LIMIT: f32 = 70.0;
    // Each `!(a > b)` / `!(a < b)` matches one `comiss`+`jae` pair, including
    // the unordered (NaN) case, which also exits here.
    if !(x > LO) || !(x < HI) || !(y > LO) || !(y < HI) {
        return 0.0;
    }
    unsafe {
        let count = *global::<i32>(0x1550EAC);
        let entries = global::<u8>(0x1550EB4);
        let table = global::<u8>(0x158E860);
        let mut best_d = BIG;
        let mut best_v = 0.0f32;
        // Staged best value. The original seeds this slot with BIG (not 0.0)
        // and reloads it down one loop path; the seed is observable only when
        // no iteration improves on BIG, so it is reproduced exactly.
        let mut slot = BIG;
        let mut min6 = BIG;
        if count > 0 {
            let mut k: i32 = 0;
            while k < count {
                let e = entries.offset(k as isize * 16);
                let i0 = core::ptr::read_unaligned(e as *const i16) as i32;
                let t0 = table.offset(i0 as isize * 8);
                // Distance of x to [x1, x2]: 0 inside, edge gap outside.
                let x1 = core::ptr::read_unaligned(t0 as *const i16) as f32;
                let dx = if x1 > x {
                    x1 - x
                } else {
                    let i1 = core::ptr::read_unaligned(e.offset(2) as *const i16) as i32;
                    let x2 =
                        core::ptr::read_unaligned(table.offset(i1 as isize * 8) as *const i16)
                            as f32;
                    if x > x2 { x - x2 } else { 0.0 }
                };
                // Distance of y to [y1, y2], same shape.
                let y1 = core::ptr::read_unaligned(t0.offset(2) as *const i16) as f32;
                let dy = if y1 > y {
                    y1 - y
                } else {
                    let i2 = core::ptr::read_unaligned(e.offset(4) as *const i16) as i32;
                    let y2 = core::ptr::read_unaligned(
                        table.offset(i2 as isize * 8).offset(2) as *const i16,
                    ) as f32;
                    let d = if y > y2 { y - y2 } else { 0.0 };
                    // Down this path the original clobbers its value
                    // register with y2 and reloads it from the slot.
                    best_v = slot;
                    d
                };
                let d = (dy * dy + dx * dx).sqrt();
                // Strict improvement only; NaN distances keep the incumbent.
                if best_d > d {
                    best_v = core::ptr::read_unaligned(t0.offset(4) as *const f32);
                    best_d = d;
                    slot = best_v;
                }
                if min6 > d {
                    min6 = d;
                }
                k += 1;
            }
        }
        slot = best_d;
        let _ = slot;
        if (flags & 0xFF) != 1 {
            return best_d;
        }
        let adjust = *global::<f32>(0x128E348);
        let diff = adjust - best_v;
        *global::<f32>(0x154EC58) = best_v;
        let t = if 0.0 > diff { 0.0 } else { diff };
        let r = (t * t + min6 * min6).sqrt();
        if r > LIMIT {
            *global::<u32>(0x154EC58) = 0;
        }
        *global::<f32>(0x103F4C8) = best_d;
        best_d
    }
});
