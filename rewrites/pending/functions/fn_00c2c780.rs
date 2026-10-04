// original: 0x00c2c780 audfire_bind_emit
// Shared helpers (identical in this lane's other rewrite files; keep one copy
// when merging): fadd_first reproduces one addss with first-operand NaN
// precedence, which Rust codegen does not otherwise pin.
#[inline(always)]
unsafe fn store_f(addr: u32, v: f32) {
    *(addr as *mut f32) = v;
}

#[inline(always)]
fn fadd_first(a: f32, b: f32) -> f32 {
    let ai = a.to_bits();
    if ai & 0x7F800000 == 0x7F800000 && ai & 0x007FFFFF != 0 {
        return a;
    }
    let bi = b.to_bits();
    if bi & 0x7F800000 == 0x7F800000 && bi & 0x007FFFFF != 0 {
        return b;
    }
    a + b
}

/// Binds the emitter at `this` (center at +0, radius at +0x10) against the two
/// points `a` and `b`. Builds a bound object (callee 1), stages the
/// origin-relative offsets plus three global tuning floats into working
/// buffers, and runs the intersection test (callee 2). If the test reports a
/// hit, writes two hit records (center plus tuning offsets, and the w value
/// the test produced) to `out1` and `out2`; otherwise writes nothing. Always
/// tears the bound object down (callees 3 and 4) and returns 1 on a hit, 0 on
/// a miss (only AL significant).
///
/// Callee identities in the contract: 1 = bound-object construct (thiscall/1,
/// radius argument), 2 = intersection test (thiscall/5: three working-buffer
/// pointers plus the constants 2.0 and -1.0; writes the w word through the
/// third buffer), 3 = bound-object release (thiscall/1), 4 = bound-object
/// destroy (thiscall/0).
export!(thiscall, rw_00c2c780(this: u32, a: u32, b: u32, out1: u32, out2: u32) -> u32 {
    unsafe {
        let tx = *(this as *const f32);
        let ty = *((this.wrapping_add(4)) as *const f32);
        let tz = *((this.wrapping_add(8)) as *const f32);
        let radius = *((this.wrapping_add(0x10)) as *const f32);
        let mut obj = 0u32;
        let objp = (&mut obj as *mut u32) as u32;
        let _: u32 = callee_thiscall!(1, u32, objp, radius.to_bits());
        let g_mid = *global::<f32>(0x1B4B324);
        let g_hi = *global::<f32>(0x1B4B328);
        let g_lo = *global::<f32>(0x1B4B320);
        let dbx = *(b as *const f32) - tx;
        let dby = *((b.wrapping_add(4)) as *const f32) - ty;
        let dbz = *((b.wrapping_add(8)) as *const f32) - tz;
        let a0 = *((a.wrapping_add(8)) as *const f32) - tz;
        let dax = *(a as *const f32) - tx;
        let day = *((a.wrapping_add(4)) as *const f32) - ty;
        let mut p3 = [dax.to_bits(), day.to_bits(), a0.to_bits(), 0u32, dbx.to_bits(), dby.to_bits(), dbz.to_bits()];
        let mut p1 = 0u32;
        let mut p2 = 0u32;
        // Stack-argument order is last-pushed-first: (p3, p2, p1, 2.0, -1.0).
        let hit: u32 = callee_thiscall!(
            2, u32, objp,
            p3.as_mut_ptr() as u32,
            (&mut p2 as *mut u32) as u32,
            (&mut p1 as *mut u32) as u32,
            0x40000000, 0xBF800000u32
        );
        let flag: u8;
        if hit != 0 {
            let w = f32::from_bits(p3[3]);
            store_f(out1, fadd_first(tx, g_lo));
            store_f(out1.wrapping_add(4), fadd_first(ty, g_mid));
            store_f(out1.wrapping_add(8), fadd_first(tz, g_hi));
            store_f(out1.wrapping_add(12), w);
            store_f(out2, fadd_first(tx, g_lo));
            store_f(out2.wrapping_add(4), fadd_first(g_mid, ty));
            store_f(out2.wrapping_add(8), fadd_first(g_hi, tz));
            store_f(out2.wrapping_add(12), w);
            let _: u32 = callee_thiscall!(3, u32, objp, 0);
            flag = 1;
        } else {
            let _: u32 = callee_thiscall!(3, u32, objp, 0);
            flag = 0;
        }
        let _: u32 = callee_thiscall!(4, u32, objp);
        flag as u32
    }
});
