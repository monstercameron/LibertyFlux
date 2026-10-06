// original: 0x00b03de0 point_in_volumes_pos
/// Point-in-volumes test with a direct threshold.
///
/// thiscall `(this, index, pt, thr)`: like its sibling above, but the
/// threshold is the argument as-is, the second product is ordered
/// `P1*Y`, and the group decision differs: a group with `m == 0` passes
/// vacuously (returns 1 at once), a group with `m < 0` is skipped, and
/// only a group whose planes all pass (or `m == 0`) returns; any failure
/// moves to the next group and exhaustion returns 0. The low return byte
/// is the decision (1 pass, 0 fail); the upper 24 bits carry bookkeeping
/// or, on the early-exit path, the incoming return register, so the
/// contract compares the low byte only. The stack check is off for the
/// same spilled-slot reason as its sibling.
export!(thiscall, rw_00b03de0(this: u32, index: u32, pt: u32, thr: u32) -> u32 {
    const STRIDE: u32 = 0x5580;
    const COUNT_OFF: u32 = 0x4000;
    unsafe {
        let table = (this as *const u32).read_unaligned();
        let base = table.wrapping_add(index.wrapping_mul(STRIDE));
        let n = ((base + COUNT_OFF) as *const u32).read_unaligned();
        if (n as i32) <= 0 {
            return 0;
        }
        let px = f32::from_bits((pt as *const u32).read_unaligned());
        let py = f32::from_bits(((pt + 4) as *const u32).read_unaligned());
        let pz = f32::from_bits(((pt + 8) as *const u32).read_unaligned());
        let t = f32::from_bits(thr);
        let nn = n as i32;
        let mut g = base.wrapping_add(0xEC);
        let mut i = 0i32;
        while i < nn {
            if ((g.wrapping_sub(0xC)) as *const u32).read_unaligned() == 0 {
                let m = (g as *const u32).read_unaligned();
                if (m as i32) <= 0 {
                    if m == 0 {
                        return 1;
                    }
                } else {
                    let mm = m as i32;
                    let mut p = g.wrapping_sub(0xE8);
                    let mut j = 0i32;
                    let mut passed = false;
                    loop {
                        let p0 = f32::from_bits(((p.wrapping_sub(4)) as *const u32).read_unaligned());
                        let p1 = f32::from_bits((p as *const u32).read_unaligned());
                        let p2 = f32::from_bits(((p + 4) as *const u32).read_unaligned());
                        let p3 = f32::from_bits(((p + 8) as *const u32).read_unaligned());
                        let t1 = core::hint::black_box(p0) * core::hint::black_box(px);
                        let t2 = core::hint::black_box(p1) * core::hint::black_box(py);
                        let t3 = core::hint::black_box(t1) + core::hint::black_box(t2);
                        let t4 = core::hint::black_box(p2) * core::hint::black_box(pz);
                        let t5 = core::hint::black_box(t3) + core::hint::black_box(t4);
                        let v = core::hint::black_box(t5) - core::hint::black_box(p3);
                        if v > t {
                            break;
                        }
                        j += 1;
                        p = p.wrapping_add(0x10);
                        if j >= mm {
                            passed = true;
                            break;
                        }
                    }
                    if passed {
                        return (m & 0xFFFF_FF00) | 1;
                    }
                }
            }
            i += 1;
            g = g.wrapping_add(0x100);
        }
        0
    }
});
