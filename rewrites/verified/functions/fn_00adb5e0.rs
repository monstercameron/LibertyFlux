// original: 0x00adb5e0 grid_cell_blend (proposed)

/// Interpolate a value over one grid cell and test it against a reference.
///
/// `cell` points to three signed 16-bit vertex indices (`+0`, `+2`, `+4`)
/// and a flag byte at `+6` (bit `0x04`). Each index addresses an 8-byte
/// entry in the grid table (base `GRID_TABLE`): a signed 16-bit X at `+0`,
/// a signed 16-bit Y at `+2`, a float Z at `+4`.
///
/// The point (`u`, `v`) must fall inside the cell: `X0 <= u <= X1` and
/// `min(Y0, Y2) <= v <= max(Y0, Y2)`, where the bounds come from entries
/// 0, 1 and 2. Normalised coordinates are `up = (u - X0) / (X1 - X0)` and
/// `vp = (v - Y0) / (Y2 - Y0)`. When entry 2 shares entry 0's X the cell
/// is a triangle: it additionally requires `up + vp <= LIMIT` (the
/// constant at `LIMIT_CONST`, 1.0) and the result is
/// `Z0 + (Z1 - Z0) * up + (Z2 - Z0) * vp`. Otherwise the second triangle
/// requires `up >= vp` and the result is
/// `Z1 + (Z0 - Z1) * (LIMIT - up) + (Z2 - Z1) * vp`.
///
/// The result is stored to `out`, then `result - refr` is compared with
/// `tol`: a difference strictly above `tol` fails when the flag bit is
/// set. The return is 1 when `tol <= result + bias`, else 0. Any NaN
/// input fails closed (returns 0) except that a NaN tolerance or NaN
/// `result + bias` compares unordered-equal and returns 1, matching the
/// original's `comiss`/`setbe` pair.
///
/// Original: 0x00ADB5E0 (cdecl, seven stack words). Only the low byte of
/// the return is meaningful; the upper bytes are leftovers.
lf_checker_rt::export!(cdecl, rw_00adb5e0(cell: u32, u: u32, v: u32, tol: u32, out: u32, refr: u32, bias: u32) -> u32 {
    unsafe {
        const GRID_TABLE: u32 = 0x0158_E860;
        const LIMIT_CONST: u32 = 0x00FE_88E8;
        const FLAG_BIT: u8 = 0x04;

        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let tbase = lf_checker_rt::relocated(GRID_TABLE);
        let ix0 = rd16(cell) as i16 as i32;
        let ix1 = rd16(cell.wrapping_add(2)) as i16 as i32;
        let ix2 = rd16(cell.wrapping_add(4)) as i16 as i32;
        let e0 = tbase.wrapping_add(ix0.wrapping_mul(8) as u32);
        let e1 = tbase.wrapping_add(ix1.wrapping_mul(8) as u32);
        let e2 = tbase.wrapping_add(ix2.wrapping_mul(8) as u32);
        let x0 = rd16(e0) as i16 as i32;
        let x0f = x0 as f32;
        let uf = f32::from_bits(u);
        if !(uf >= x0f) {
            return 0;
        }
        let x1 = rd16(e1) as i16 as i32;
        if !((x1 as f32) >= uf) {
            return 0;
        }
        let y0 = rd16(e0.wrapping_add(2)) as i16 as i32;
        let y2 = rd16(e2.wrapping_add(2)) as i16 as i32;
        let (lo, hi) = if y0 < y2 {
            (y0 as f32, y2 as f32)
        } else {
            (y2 as f32, y0 as f32)
        };
        let vf = f32::from_bits(v);
        if !(vf >= lo) {
            return 0;
        }
        if !(hi >= vf) {
            return 0;
        }
        let up = div(sub(uf, x0f), (x1.wrapping_sub(x0)) as f32);
        let vp = div(sub(vf, y0 as f32), (y2.wrapping_sub(y0)) as f32);
        let x2 = rd16(e2);
        let r = if x2 == x0 as u16 {
            let c = f32::from_bits(rd32(lf_checker_rt::relocated(LIMIT_CONST)));
            if !(c >= add(vp, up)) {
                return 0;
            }
            let z0 = f32::from_bits(rd32(e0.wrapping_add(4)));
            let z1 = f32::from_bits(rd32(e1.wrapping_add(4)));
            let z2 = f32::from_bits(rd32(e2.wrapping_add(4)));
            add(add(mul(sub(z1, z0), up), z0), mul(sub(z2, z0), vp))
        } else {
            if !(up >= vp) {
                return 0;
            }
            let z1 = f32::from_bits(rd32(e1.wrapping_add(4)));
            let z0 = f32::from_bits(rd32(e0.wrapping_add(4)));
            let z2 = f32::from_bits(rd32(e2.wrapping_add(4)));
            let c = f32::from_bits(rd32(lf_checker_rt::relocated(LIMIT_CONST)));
            add(add(mul(sub(z0, z1), sub(c, up)), z1), mul(sub(z2, z1), vp))
        };
        (out as *mut u32).write_unaligned(r.to_bits());
        let tolf = f32::from_bits(tol);
        if sub(r, f32::from_bits(refr)) > tolf {
            let flags = (cell as *const u8).add(6).read();
            if flags & FLAG_BIT != 0 {
                return 0;
            }
        }
        (!(tolf > add(r, f32::from_bits(bias)))) as u32
    }
});
