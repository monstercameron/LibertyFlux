// original: 0x00988f50 positional voice updater (proposed)
//! Positional voice updater: unless the audio system is unavailable, refreshes
//! the entity's voice from the listener geometry through a chain of
//! projection, audibility and dispatch tests, exiting early whenever a test
//! fails.
//!
//! `p` and `q` point at the two endpoint vectors, `r` is recorded with the
//! dispatched voice. Returns the dispatch answer on the full path, the `q`
//! pointer on geometry exits, else the failing probe's answer.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
fn g32(va: u32) -> u32 {
    unsafe { *global::<u32>(va) }
}

#[inline(always)]
fn gf(va: u32) -> f32 {
    unsafe { *global::<f32>(va) }
}

#[inline(always)]
fn rf(base: u32, off: u32) -> f32 {
    unsafe { *((base.wrapping_add(off)) as *const f32) }
}

/// Emulates `cvttss2si` (truncate toward zero, out-of-range and NaN give
/// 0x80000000) followed by taking the low 16 bits as signed.
#[inline(always)]
fn cvtt_low16(x: f32) -> u32 {
    let v: u32 = if x > -2147483649.0 && x < 2147483648.0 {
        (x as i32) as u32
    } else {
        0x80000000
    };
    ((v as i16) as i32) as u32
}

// original: 0x00988f50 positional voice updater (proposed)
export!(thiscall, rw_00988f50(this: u32, p: u32, q: u32, r: u32) -> u32 {
    if g32(0x11f7060) != 1 && g32(0x12088b4) == g32(0xf1c040) && g32(0x1037720) != 0x12 {
        callee_cdecl!(1, u32, p, q);
    }
    let (l0, l1, l2) = (gf(0x128e320), gf(0x128e324), gf(0x128e328));
    let c0 = gf(0xfe88e8);
    let s = (l1 * l1 + l0 * l0) + l2 * l2;
    let k = if s != 0.0 { c0 / s.sqrt() } else { 0.0 };
    let px = rf(p, 0);
    let py = rf(p, 4);
    let pz = rf(p, 8);
    let dx = rf(q, 0) - px;
    let dy = rf(q, 4) - py;
    let dz = rf(q, 8) - pz;
    let c1 = gf(0xfe8830);
    let a10 = (k * l0) * c1 + gf(0x128e340);
    let a118 = (l2 * k) * c1 + gf(0x128e348);
    let a60 = (l1 * k) * c1 + gf(0x128e344);
    let d = (dy * dy + dx * dx) + dz * dz;
    let n = if d != 0.0 { c0 / d.sqrt() } else { 0.0 };
    let (nx, ny, nz) = (dx * n, dy * n, dz * n);
    let mut b30 = [0.0f32; 4];
    let mut b40 = [0.0f32; 4];
    let b70 = [nx, ny, nz];
    let mut b110 = [a10, a60, a118, 0.0f32];
    let ans2 = callee_thiscall!(
        2, u32, b110.as_mut_ptr() as u32, p, b70.as_ptr() as u32, b40.as_mut_ptr() as u32,
        b30.as_mut_ptr() as u32
    );
    if ans2 & 0xff == 0 {
        return ans2;
    }
    let dist2 = (dy * dy + dx * dx) + dz * dz;
    let g2v = ((b40[1] - py) * (b40[1] - py) + (b40[0] - px) * (b40[0] - px))
        + (b40[2] - pz) * (b40[2] - pz);
    if !(dist2 > g2v) {
        return q;
    }
    let g3v = ((b30[0] - px) * (b30[0] - px) + (b30[1] - py) * (b30[1] - py))
        + (b30[2] - pz) * (b30[2] - pz);
    if !(dist2 > g3v) {
        return q;
    }
    let dot4 = ((b40[1] - py) * ny + (b40[0] - px) * nx) + (b40[2] - pz) * nz;
    if !(dot4 >= 0.0) {
        return q;
    }
    let dot5 = ((b30[1] - py) * ny + (b30[0] - px) * nx) + (b30[2] - pz) * nz;
    if !(dot5 >= 0.0) {
        return q;
    }
    let ans3a = callee_cdecl!(3, u32,);
    let flag = g32(0x11f70cc);
    let t0 = g32(0x11735b4);
    let esi_v: u32;
    let mut cmp_eax = 0u32;
    let mut skip_compare = false;
    if ans3a & 0xff != 0 && flag != 0 {
        esi_v = t0;
        skip_compare = true;
    } else if ans3a & 0xff != 0 {
        let a4 = callee_cdecl!(5, u32, 0x3e800000u32);
        if a4 & 0xff != 0 {
            esi_v = t0;
            skip_compare = true;
        } else {
            esi_v = t0;
            cmp_eax = a4;
        }
    } else {
        esi_v = t0;
        cmp_eax = ans3a;
    }
    let this_c = unsafe { *(this.wrapping_add(0xc) as *const u32) };
    if !skip_compare && esi_v < this_c {
        return cmp_eax;
    }
    let a5 = callee_cdecl!(6, u32, g32(0x1038b50), g32(0x1038b54));
    unsafe { *(this.wrapping_add(0xc) as *mut u32) = a5.wrapping_add(esi_v) };
    let b44_old = b40[1];
    let b40_old = b40[0];
    b40[1] = b44_old - a60;
    b40[0] = b40_old - a10;
    b40[3] = 0.0;
    b40[2] = 0.0;
    callee_thiscall!(9, u32, b40.as_mut_ptr() as u32);
    b30[1] = b30[1] - a60;
    b30[0] = b30[0] - a10;
    b30[2] = 0.0;
    b30[3] = 0.0;
    callee_thiscall!(9, u32, b30.as_mut_ptr() as u32);
    let f1in = (b40[0] * l0 + b40[1] * l1) + b40[2] * l2;
    let f1 = f32::from_bits(callee_cdecl!(10, u32, f1in.to_bits()));
    let m = gf(0xe7c2a8);
    let mut s10 = f1 * m;
    let f2in = (b30[0] * l0 + b30[1] * l1) + b30[2] * l2;
    let f2 = f32::from_bits(callee_cdecl!(10, u32, f2in.to_bits()));
    let mut s2c = f2 * m;
    let (g6, g7, g9) = (gf(0x128e310), gf(0x128e314), gf(0x128e318));
    let w2 = (b40[0] * g6 + b40[1] * g7) + b40[2] * g9;
    let c2v = gf(0xfe8c1c);
    if 0.0f32 > w2 {
        s10 = c2v - s10;
    }
    let w0 = (b30[1] * g7 + b30[0] * g6) + b30[2] * g9;
    if 0.0f32 > w0 {
        s2c = c2v - s2c;
    }
    let ans3b = callee_cdecl!(4, u32,);
    let flag2 = g32(0x11f70cc);
    let path_a = if ans3b & 0xff == 0 {
        flag2 == 3
    } else if flag2 == 4 {
        true
    } else {
        flag2 == 3
    };
    let last = g32(0x12831e0);
    let mut bc8 = [0u32; 4];
    if path_a {
        let a6: u32;
        loop {
            let v = callee_cdecl!(7, u32, 0u32, 2u32);
            if v != last {
                a6 = v;
                break;
            }
        }
        unsafe { *global::<u32>(0x12831e0) = a6 };
        callee_thiscall!(11, u32, bc8.as_mut_ptr() as u32);
        let _c1 = cvtt_low16(s10);
        let _rr = r;
        let tab = relocated(0x1283208).wrapping_add(a6.wrapping_mul(8));
        let tab1 = unsafe { *(tab as *const u32) };
        callee_thiscall!(12, u32, this, tab1, bc8.as_mut_ptr() as u32, 0xffffffffu32, 0u32, 0u32);
        let _c2 = cvtt_low16(s2c);
        let tab2 = unsafe { *((tab.wrapping_add(4)) as *const u32) };
        callee_thiscall!(12, u32, this, tab2, bc8.as_mut_ptr() as u32, 0xffffffffu32, 0u32, 0u32)
    } else {
        let a7: u32;
        loop {
            let v = callee_cdecl!(8, u32, 0u32, 0x0eu32);
            if v != last {
                a7 = v;
                break;
            }
        }
        unsafe { *global::<u32>(0x12831e0) = a7 };
        callee_thiscall!(11, u32, bc8.as_mut_ptr() as u32);
        let _c1 = cvtt_low16(s10);
        let _rr = r;
        let tab = relocated(0x1283220).wrapping_add(a7.wrapping_mul(8));
        let tab1 = unsafe { *(tab as *const u32) };
        callee_thiscall!(12, u32, this, tab1, bc8.as_mut_ptr() as u32, 0xffffffffu32, 0u32, 0u32);
        let _c2 = cvtt_low16(s2c);
        let tab2 = unsafe { *((tab.wrapping_add(4)) as *const u32) };
        callee_thiscall!(12, u32, this, tab2, bc8.as_mut_ptr() as u32, 0xffffffffu32, 0u32, 0u32)
    }
});
