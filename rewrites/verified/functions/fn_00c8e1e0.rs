// original: 0x00C8E1E0 ped_task_solve_mount (proposed)

/// Solve a ped-task mount query against a task object and write the result.
///
/// `obj` points to the task object (flag dword at `+0x00`, a middle object
/// at `+0x10`, an identity dword at `+0x14`). `id` must equal the identity
/// dword or the function returns 0. Otherwise the low three flag bits minus
/// one select the path: 0 solves, 1 and 2 solve through a checked middle
/// object, 3 and 4 through an unchecked one, anything else returns 0.
///
/// The solving path normalises the two floats at `dir2` (a zero length maps
/// to a zero scale, anything else to one over the length), fetches two
/// vectors through the middle object's virtual slots `VT0`/`VT1` (thiscall,
/// no stack arguments), and passes them with the direction to callee 2
/// (cdecl), which fills two four-word scratch blocks. It then combines the
/// scratch maxima with dot products against the row at `middle + ROW_OFF`
/// and writes four floats to `out`: the adjusted base point
/// (`row + ROW_BASE`) minus the accumulators and the scaled direction, plus
/// the second scratch maximum. When `extra` is non-null, callee 3 (cdecl)
/// observes `(obj, dir2, out, extra)`. The checked/unchecked paths instead
/// call callee 4 (thiscall) with `(obj, out, scratch)` and likewise report
/// through callee 3 when `extra` is set. Returns 1 on success, 0 on every
/// rejection.
///
/// The scale branch is a parity trick over an unordered-compare of the
/// squared length against zero: equal maps to zero, greater or unordered
/// (NaN) takes the square-root path, which is exactly `length == 0.0`.
/// NaN comparisons take the same branches on both sides (see the helpers).
/// A null row pointer faults while being read, before the null test below
/// it; the rewrite reads in the same order so the fault matches.
///
/// Original: 0x00C8E1E0 (cdecl, five stack words; only the low byte of the
/// result is defined).
lf_checker_rt::export!(cdecl, rw_00C8E1E0(
    obj: u32,
    id: u32,
    dir2: u32,
    out: u32,
    extra: u32,
) -> u32 {
    unsafe {
        const OBJ_MID: u32 = 0x10;
        const OBJ_ID: u32 = 0x14;
        const VT0: u32 = 0x60;
        const VT1: u32 = 0x64;
        const MID_ROW: u32 = 0x20;
        const ROW_BASE: u32 = 0x30;
        const ONE: u32 = 0xFE88E8;
        const ABS_MASK: u32 = 0xFE8F80;
        const DIR_SCALE: u32 = 0xFE880C;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// `comiss p, q` + `ja`: taken only on ordered-above.
        #[inline(always)]
        fn above(p: f32, q: f32) -> bool {
            p > q
        }
        #[inline(always)]
        fn abs_bits(v: f32, mask: u32) -> f32 {
            f32::from_bits(v.to_bits() & mask)
        }

        if rd32(obj.wrapping_add(OBJ_ID)) != id {
            return 0;
        }
        let sel = (rd32(obj) & 7).wrapping_sub(1);
        if sel == 0 {
            let mid = rd32(obj.wrapping_add(OBJ_MID));
            if mid == 0 {
                return 0;
            }
            let dx = rdf(dir2);
            let dy = rdf(dir2.wrapping_add(4));
            let yy = mul(dy, dy);
            let xx = mul(dx, dx);
            let len2 = add(yy, xx);
            // Parity trick over ucomiss(len2, 0): equal -> 0, else 1/sqrt.
            let scale = if len2 == 0.0 {
                0.0
            } else {
                div(rdf(lf_checker_rt::relocated(ONE)), len2.sqrt())
            };
            let dir_x = mul(dx, scale);
            let dir_y = mul(dy, scale);
            let dir_z = mul(scale, 0.0);
            let vt = rd32(mid);
            let get0: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vt.wrapping_add(VT0)) as usize) };
            let p1 = get0(mid);
            let v1x = rdf(p1);
            let v1y = rdf(p1.wrapping_add(4));
            let get1: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vt.wrapping_add(VT1)) as usize) };
            let p2 = get1(mid);
            let v2x = rdf(p2);
            let v2y = rdf(p2.wrapping_add(4));
            // Scratch blocks, zero-initialised like the original's frame
            // slots (a callee snapshot compares their pre-call contents).
            let mut b1 = [0u32; 4];
            let mut b2 = [0u32; 4];
            b1[0] = v1x.to_bits();
            b1[1] = v1y.to_bits();
            b2[0] = v2x.to_bits();
            b2[1] = v2y.to_bits();
            lf_checker_rt::callee_cdecl!(
                2, u32, mid, (&mut b1 as *mut u32) as u32, (&mut b2 as *mut u32) as u32
            );
            // The first-maximum inputs are read before the callee's stack
            // slots are released, so they are the blocks' first words.
            let g0 = f32::from_bits(b1[0]);
            let g1 = f32::from_bits(b1[1]);
            let h0 = f32::from_bits(b2[0]);
            let h1 = f32::from_bits(b2[1]);
            let h3 = f32::from_bits(b2[3]);
            let mask = rd32(lf_checker_rt::relocated(ABS_MASK));
            let t0 = abs_bits(g0, mask);
            let m1 = if above(h0, t0) { h0 } else { t0 };
            let row = rd32(mid.wrapping_add(MID_ROW));
            // The row is dereferenced before the null test below; keep the
            // order so a null row faults identically on both sides.
            let r0 = rdf(row);
            let r4 = rdf(row.wrapping_add(4));
            let r8 = rdf(row.wrapping_add(8));
            let r10 = rdf(row.wrapping_add(0x10));
            let r14 = rdf(row.wrapping_add(0x14));
            let r18 = rdf(row.wrapping_add(0x18));
            let t1 = abs_bits(g1, mask);
            let m2 = if above(h1, t1) { h1 } else { t1 };
            let mut dot = mul(r4, dir_y);
            dot = add(dot, mul(r0, dir_x));
            dot = add(dot, mul(r8, dir_z));
            dot = mul(dot, m1);
            let mut p = mul(r14, dir_y);
            p = add(p, mul(r10, dir_x));
            p = add(p, mul(r18, dir_z));
            p = mul(p, m2);
            let ox = add(mul(r0, dot), mul(r10, p));
            let oy = add(mul(r4, dot), mul(r14, p));
            let oz = add(mul(r8, dot), mul(r18, p));
            let k = rdf(lf_checker_rt::relocated(DIR_SCALE));
            let sx = mul(dir_x, k);
            let sy = mul(dir_y, k);
            let sz = mul(dir_z, k);
            let base = if row == 0 {
                mid.wrapping_add(0x10)
            } else {
                row.wrapping_add(ROW_BASE)
            };
            let mut fx = rdf(base);
            let mut fy = rdf(base.wrapping_add(4));
            let mut fz = rdf(base.wrapping_add(8));
            fx = core::hint::black_box(fx) - core::hint::black_box(ox);
            fy = core::hint::black_box(fy) - core::hint::black_box(oy);
            fz = core::hint::black_box(fz) - core::hint::black_box(oz);
            fx = core::hint::black_box(fx) - core::hint::black_box(sx);
            fy = core::hint::black_box(fy) - core::hint::black_box(sy);
            fz = core::hint::black_box(fz) - core::hint::black_box(sz);
            wrf(out, fx);
            wrf(out.wrapping_add(4), fy);
            wrf(out.wrapping_add(8), fz);
            wrf(out.wrapping_add(0xc), h3);
            if extra != 0 {
                lf_checker_rt::callee_cdecl!(3, u32, obj, dir2, out, extra);
            }
            1
        } else if sel == 1 || sel == 2 {
            if rd32(obj.wrapping_add(OBJ_MID)) == 0 {
                return 0;
            }
            let mut scratch = [0u32; 3];
            lf_checker_rt::callee_thiscall!(
                4, u32, obj, out, (&mut scratch as *mut u32) as u32
            );
            if extra != 0 {
                lf_checker_rt::callee_cdecl!(3, u32, obj, dir2, out, extra);
            }
            1
        } else if sel == 3 || sel == 4 {
            let mut scratch = [0u32; 3];
            lf_checker_rt::callee_thiscall!(
                4, u32, obj, out, (&mut scratch as *mut u32) as u32
            );
            if extra != 0 {
                lf_checker_rt::callee_cdecl!(3, u32, obj, dir2, out, extra);
            }
            1
        } else {
            0
        }
    }
});
