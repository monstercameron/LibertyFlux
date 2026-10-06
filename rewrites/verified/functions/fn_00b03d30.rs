// original: 0x00b03d30 point_in_volumes_neg
/// Point-in-volumes test with a negated threshold.
///
/// thiscall `(this, index, pt, thr, _, _)`: `base = [this] + index*0x5580`
/// selects a table row holding a group count `n` (SIGNED 32-bit at
/// `+0x4000`; `n <= 0` returns `n` with its low byte cleared). Each group
/// `i` (at `base+0xec+i*0x100`) is skipped when its flag word (at
/// `-0xc`) is nonzero; otherwise its plane count `m` (SIGNED at `+0`)
/// decides: `m <= 0` returns 1 at once, else every plane `j` (five words
/// at `g-0xec+j*0x10`, offsets `-4,0,4,8` past the cursor) must satisfy
/// `P0*X + Y*P1 + P2*Z - P3 <= -thr`, evaluated in exactly that order
/// (the threshold is the sign-flipped argument; an unordered NaN result
/// passes, matching the original's unsigned-above jump). The first group
/// whose planes all pass returns `m` with its low byte set to 1; when no
/// group passes, returns `n` with its low byte cleared. The original
/// spills `n` into its own incoming argument slot; the contract switches
/// the stack check off and observes `n` through the trip counts and the
/// return value instead.
export!(thiscall, rw_00b03d30(this: u32, index: u32, pt: u32, thr: u32, _a3: u32, _a4: u32) -> u32 {
    const STRIDE: u32 = 0x5580;
    const COUNT_OFF: u32 = 0x4000;
    const SIGN_BIT: u32 = 0x8000_0000;
    unsafe {
        let table = (this as *const u32).read_unaligned();
        let base = table.wrapping_add(index.wrapping_mul(STRIDE));
        let n = ((base + COUNT_OFF) as *const u32).read_unaligned();
        if (n as i32) <= 0 {
            return n & 0xFFFF_FF00;
        }
        let px = f32::from_bits((pt as *const u32).read_unaligned());
        let py = f32::from_bits(((pt + 4) as *const u32).read_unaligned());
        let pz = f32::from_bits(((pt + 8) as *const u32).read_unaligned());
        let t = f32::from_bits(thr ^ SIGN_BIT);
        let nn = n as i32;
        let mut g = base.wrapping_add(0xEC);
        let mut i = 0i32;
        while i < nn {
            if ((g.wrapping_sub(0xC)) as *const u32).read_unaligned() == 0 {
                let m = (g as *const u32).read_unaligned();
                if (m as i32) <= 0 {
                    return 1;
                }
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
                    let t2 = core::hint::black_box(py) * core::hint::black_box(p1);
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
            i += 1;
            g = g.wrapping_add(0x100);
        }
        n & 0xFFFF_FF00
    }
});
