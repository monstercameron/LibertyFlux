// original: 0x009748c0 aud_occlusion_attenuation
/// Audio occlusion attenuation: three-axis table comparison folded to a gain.
///
/// Takes two selector bytes (only the low byte of each stack argument is
/// read) and a pointer to three floats. Each float is compared against a pair
/// of runtime-filled tables at row `(a + 8 * b) * 16` bytes; the comparison
/// direction flips per axis depending on the selectors. Returns `0.0` (full
/// silence) when all three first-table checks hit, `0.5` when at least one
/// check per axis hits, else `1.0`.
///
/// Quirk (observed): the second table's checks on the third axis compare
/// against the *second* float, not the third, in both selector branches.
/// The checker-caught mutant proves the original really does this.
///
/// The original also clobbers the low byte of two of its own incoming stack
/// slots (callee-popped scratch); that clobber is not reproduced here and the
/// `stack` comparison is disabled in the contract for that reason.
export!(stdcall, rw_009748c0(a: u32, b: u32, p: u32) -> f32 {
    let a = a as u8;
    let b = b as u8;
    let p = p as *const f32;
    // Row selected by the two bytes: (a + 8*b) * 16.
    let row = ((a as u32) + 8 * (b as u32)) * 16;
    let t1 = relocated(0x01165e30) as *const u8;
    let t2 = relocated(0x01166030) as *const u8;
    unsafe {
        let at = |base: *const u8, off: u32| -> f32 {
            *(base.add(off as usize) as *const f32)
        };
        let x = *p;
        let y = *p.add(1);
        let z = *p.add(2);
        let t1x = at(t1, row);
        let t2x = at(t2, row);
        let t1y = at(t1, row + 4);
        let t2y = at(t2, row + 4);
        let t1z = at(t1, row + 8);
        let t2z = at(t2, row + 8);

        // First axis: direction flips on a < 4.
        let (d, f0) = if a < 4 {
            if t1x > x {
                (1u8, true)
            } else if t2x > x {
                (1, false)
            } else {
                (0, false)
            }
        } else if x > t1x {
            (1, true)
        } else if x > t2x {
            (1, false)
        } else {
            (0, false)
        };
        // Second axis: direction flips on (a - 2) <= 3 (wrapping).
        let (e, f1) = if a.wrapping_sub(2) <= 3 {
            if y > t1y {
                (1u8, true)
            } else if y > t2y {
                (1, false)
            } else {
                (0, false)
            }
        } else if t1y > y {
            (1, true)
        } else if t2y > y {
            (1, false)
        } else {
            (0, false)
        };
        // Third axis: direction flips on b < 2. Note the original compares
        // the second table against y here, not z.
        let (g, h) = if b < 2 {
            if t1z > z {
                (1u8, 1u8)
            } else if t2z > y {
                (1, 0)
            } else {
                (0, 0)
            }
        } else if z > t1z {
            (1, 1)
        } else if y > t2z {
            (1, 0)
        } else {
            (0, 0)
        };

        if f0 && f1 && h != 0 {
            0.0
        } else if d != 0 && e != 0 && g != 0 {
            0.5
        } else {
            1.0
        }
    }
});
