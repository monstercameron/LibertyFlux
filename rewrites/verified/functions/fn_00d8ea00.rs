// original: 0x00d8ea00 audio_probe_refresh
//! Refresh the child pose when the probe moved.
//!
//! Skips the work only if both direction dots clear the 0.9 threshold and the
//! position is inside the 0.2 tolerance on all three axes; otherwise rescans
//! every record through the bounds scanner and copies the fresh rows into the
//! child. Returns 0 on the early exit, else the last copied word (as the
//! original falls through with it in EAX).
//!
//! Float accumulation orders and NaN-sensitive branch polarities match the
//! original exactly; see the inline comments.
export!(thiscall, rw_00d8ea00(this: u32) -> u32 {
    unsafe {
        let e = this as *const f32;
        let child = *(this as *const u32).add(0x90 / 4);
        let c = child as *const f32;
        let thr = *global::<f32>(0xFE88BC);
        // Dots in the original accumulation order (float addition order matters).
        let d0 = (*c.add(0x34 / 4) * *e.add(0x24 / 4) + *e.add(0x20 / 4) * *c.add(0x30 / 4))
            + *c.add(0x38 / 4) * *e.add(0x28 / 4);
        if !(thr > d0) {
            let d1 = (*c.add(0x14 / 4) * *e.add(0x04 / 4) + *e.add(0x00 / 4) * *c.add(0x10 / 4))
                + *c.add(0x18 / 4) * *e.add(0x08 / 4);
            if !(thr > d1) {
                let tol = *global::<f32>(0xFE87D0);
                let mut inside = true;
                let mut k = 0usize;
                while k < 3 {
                    let v = *e.add(0x30 / 4 + k);
                    let cc = *c.add(0x40 / 4 + k);
                    if comiss_jb(v, cc - tol) || comiss_jb(cc + tol, v) {
                        inside = false;
                        break;
                    }
                    k += 1;
                }
                if inside {
                    // Early exit leaves EAX = 0 ((an instruction of the original)).
                    return 0;
                }
            }
        }
        let n = *(this as *const u32).add(0x7C / 4);
        if n > 0 {
            let base = *(this as *const u32).add(0x6C / 4);
            let mut i = 0u32;
            while i < n {
                let _: u32 =
                    callee_thiscall!(1, u32, this, base.wrapping_add(i.wrapping_mul(0x28)));
                i += 1;
            }
        }
        let eu = this as *const u32;
        let cu = child as *mut u32;
        let pairs: [(usize, usize); 12] = [
            (0x00, 0x10), (0x04, 0x14), (0x08, 0x18), (0x10, 0x20), (0x14, 0x24),
            (0x18, 0x28), (0x20, 0x30), (0x24, 0x34), (0x28, 0x38), (0x30, 0x40),
            (0x34, 0x44), (0x38, 0x48),
        ];
        let mut k = 0usize;
        while k < 12 {
            let (s, d) = pairs[k];
            *cu.add(d / 4) = *eu.add(s / 4);
            k += 1;
        }
        // Falls through with the last copied word in EAX.
        *eu.add(0x38 / 4)
    }
});

/// `comiss a, b; jb taken`: jump iff a is unordered-or-below b, i.e. unless
/// a >= b holds as an ordered comparison. NaN on either side jumps.
/// (Helper shared with the crate; included so this file stands alone.)
#[inline]
fn comiss_jb(a: f32, b: f32) -> bool {
    !(a >= b)
}
