// original: 0x00B5CB10 unnamed
/// Range-gate a weapon discharge: unless the source object or callee 1 vetos
/// (return 0), write the source-to-target difference triple to `p_out`,
/// normalize-and-scale it twice against read-only constants, then store the
/// weapon table's float at `a2` (scaled by 0.1 or 1.0 per a flag bit in the
/// `p_edi` object) to `p_outf`. Returns 1 on the full path.
///
/// The last `p_out` word comes from a frame slot the original never stores:
/// an uninitialized-stack read, reproduced here as one (the checker defines
/// it through `stack_fill`). Only the low result byte is significant.
///
/// Cond: `p_flt` points to 3 readable floats, `p_esi` (or null) to an object
/// with floats at +0x10/+0x14/+0x18, `p_edi` to an object readable at +0x2A0,
/// `p_out` to 4 writable floats, `p_outf` to 1 writable float.
export!(cdecl, rw_b73_f2(a0: u32, p_edi: u32, a2: u32, p_flt: u32, p_esi: u32, p_out: u32, p_outf: u32) -> u32 {
    unsafe {
        if p_esi == 0 {
            return 0;
        }
        let ok: u32 = callee_cdecl!(1, u32, p_edi, 0, a0, 0);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        // Difference triple, then the squared length in the original's order
        // (dx first here).
        let fa = p_flt as *const f32;
        let dx = *(p_esi.wrapping_add(0x10) as *const f32) - *fa.offset(0);
        let dy = *(p_esi.wrapping_add(0x14) as *const f32) - *fa.offset(1);
        let dz = *(p_esi.wrapping_add(0x18) as *const f32) - *fa.offset(2);
        let oc = p_out as *mut f32;
        // The original reads a frame slot it never wrote. A volatile load of
        // an uninitialized local reproduces the read itself (not any value);
        // under the checker both sides observe the defined stack fill.
        let slot = core::mem::MaybeUninit::<u32>::uninit();
        *oc.offset(3) = f32::from_bits(core::ptr::read_volatile(slot.as_ptr()));
        let len2 = addss_exact(
            addss_exact(mulss_exact(dx, dx), mulss_exact(dy, dy)),
            mulss_exact(dz, dz),
        );
        *oc.offset(0) = dx;
        *oc.offset(1) = dy;
        *oc.offset(2) = dz;
        let c1 = *global::<f32>(0xFE88E8);
        // The original's flag test skips exactly when len2 is +0 (NaN takes
        // the divide side); `!=` reproduces that shape.
        let mut inv = 0.0f32;
        if len2 != 0.0 {
            inv = c1 / len2.sqrt();
        }
        let sx = mulss_exact(inv, dx);
        let sy = mulss_exact(inv, dy);
        let sz = mulss_exact(inv, dz);
        *oc.offset(2) = sz;
        *oc.offset(0) = sx;
        *oc.offset(1) = sy;
        // Second gate over the scaled pair's length (dest is the sy side).
        let t = addss_exact(mulss_exact(sy, sy), mulss_exact(sx, sx));
        let sl = t.sqrt();
        let c005 = *global::<f32>(0xFE876C);
        if sl > c005 {
            let k = c1 / sl;
            *oc.offset(0) = mulss_exact(sx, k);
            *oc.offset(1) = mulss_exact(sy, k);
        }
        let wi: u32 = callee_cdecl!(2, u32, a2);
        let w0 = *(wi.wrapping_add(0x90) as *const f32);
        let of = p_outf as *mut f32;
        *of.offset(0) = w0;
        let flag = *(p_edi.wrapping_add(0x2A0) as *const u8);
        if (flag & 4) != 0 {
            *of.offset(0) = mulss_exact(w0, *global::<f32>(0xFE879C));
        } else {
            *of.offset(0) = mulss_exact(w0, c1);
        }
        1
    }
});
// Exact-operation helpers used above (shared across this lane's rewrites).
fn addss_exact(dest: f32, src: f32) -> f32 {
    let a = dest.to_bits();
    if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
        return f32::from_bits(a | 0x00400000);
    }
    let b = src.to_bits();
    if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
        return f32::from_bits(b | 0x00400000);
    }
    dest + src
}
fn mulss_exact(dest: f32, src: f32) -> f32 {
    let a = dest.to_bits();
    if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
        return f32::from_bits(a | 0x00400000);
    }
    let b = src.to_bits();
    if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
        return f32::from_bits(b | 0x00400000);
    }
    dest * src
}
