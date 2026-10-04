// original: 0x00B5C9B0 NativeImpl_FIRE_PED_WEAPON_2
/// Resolve a weapon descriptor through callee 1, copy it to `out_a`, copy the
/// input quad `in_c` to `out_b`, then advance `out_b`'s float triple along the
/// normalized (`out_b` minus `out_a`) direction scaled by the two scripted
/// scalar answers (callees 2 and 3).
///
/// The low descriptor word is an integer tag that the arithmetic reinterprets
/// as a float; the last `out_b` word comes from callee 1's frame-slot write.
/// Only the low result byte is significant; always 1.
///
/// Cond: `out_a`/`out_b` point to writable 16-byte quads, `in_c` to a readable
/// 16-byte quad.
export!(thiscall, rw_b73_f1(this: u32, a0: u32, a1: u32, out_a: u32, out_b: u32, in_c: u32) -> u32 {
    unsafe {
        // Callee 1 resolves `a1` into a 4-word descriptor: words come back
        // through the returned pointer, the last one through this slot.
        // Zeroed to match the checker's defined stack fill (snapshotted).
        let mut frame = [0u32; 4];
        let q: u32 = callee_thiscall!(1, u32, this, frame.as_mut_ptr() as u32, a1, 3);
        let qa = q as *const u32;
        let oa = out_a as *mut u32;
        *oa.offset(0) = *qa.offset(0);
        *oa.offset(1) = *qa.offset(1);
        *oa.offset(2) = *qa.offset(2);
        *oa.offset(3) = *qa.offset(3);
        let ca = in_c as *const u32;
        let ob = out_b as *mut u32;
        *ob.offset(0) = *ca.offset(0);
        *ob.offset(1) = *ca.offset(1);
        *ob.offset(2) = *ca.offset(2);
        *ob.offset(3) = *ca.offset(3);
        // Difference of the float triples (SUBSS is not commutable, so plain
        // `-` is exact), then the squared length in the original's order.
        let fa = out_a as *const f32;
        let fb = out_b as *mut f32;
        let dx = *fb.offset(0) - *fa.offset(0);
        let dy = *fb.offset(1) - *fa.offset(1);
        let dz = *fb.offset(2) - *fa.offset(2);
        let len2 = addss_exact(
            addss_exact(mulss_exact(dy, dy), mulss_exact(dx, dx)),
            mulss_exact(dz, dz),
        );
        let r: f32 = callee_cdecl!(2, f32, len2.to_bits());
        let nx = mulss_exact(r, dx);
        let ny = mulss_exact(r, dy);
        let nz = mulss_exact(r, dz);
        let r3: f32 = callee_thiscall!(3, f32, this, a0);
        let t1 = mulss_exact(nz, r3);
        let t2 = mulss_exact(nx, r3);
        let t3 = mulss_exact(ny, r3);
        // Note the mixed operand order: lanes 0 and 2 add onto the `out_a`
        // value, lane 1 onto the scaled product. Each needs its own side.
        *fb.offset(0) = addss_exact(*fa.offset(0), t2);
        *fb.offset(1) = addss_exact(t3, *fa.offset(1));
        *fb.offset(2) = addss_exact(*fa.offset(2), t1);
        *ob.offset(3) = frame[3];
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
