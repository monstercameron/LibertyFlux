// original: 0x00c2cea0 audfire_level_update
// Shared helpers (identical in this lane's other rewrite files; keep one copy
// when merging): fadd_first/fmul_first reproduce one addss/mulss with
// first-operand NaN precedence, which Rust codegen does not otherwise pin.
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

#[inline(always)]
fn fmul_first(a: f32, b: f32) -> f32 {
    let ai = a.to_bits();
    if ai & 0x7F800000 == 0x7F800000 && ai & 0x007FFFFF != 0 {
        return a;
    }
    let bi = b.to_bits();
    if bi & 0x7F800000 == 0x7F800000 && bi & 0x007FFFFF != 0 {
        return b;
    }
    a * b
}

/// Updates fire-audio levels for the entity at `obj`. Dispatches on the type
/// field at obj+0x48 -> +0x28 bits 6..10: kind 3 samples four channel gains
/// (each returned as a double by callee 1) and keeps the maximum, kind 2
/// skips the sampling; both then call the virtual at slot +0xEC (callee 2)
/// to get a position, derive a distance term from it, and clamp it to 1.0.
/// If the flag word instead has bits 6..10 == 2 at the end (the kind-2 case),
/// the main fire loop (callee 3) runs and the function returns; otherwise a
/// parameter block is fetched (callee 4) and the level is finished through
/// callee 5. The second stack argument is ignored. No value is returned.
///
/// Callee identities in the contract: 1 = gain sampler (thiscall/1, f64 in
/// ST0), 2 = position virtual (thiscall/1 through the object's vtable, returns
/// a float-triple pointer), 3 = main fire loop (thiscall/2), 4 = parameter
/// fetch (thiscall/1), 5 = level finish (thiscall/3).
export!(thiscall, rw_00c2cea0(this: u32, obj: u32, _ignored: u32) -> u32 {
    unsafe {
        let mut gain_cap = 1.0f32;
        let mut dist_term = 1.0f32;
        let ent = *(obj.wrapping_add(0x48) as *const u32);
        if ent != 0 {
            let kind = ((*(ent.wrapping_add(0x28) as *const u32) >> 6) & 0xF) as u8;
            if kind == 3 {
                let sampler_this = ent.wrapping_add(0x3C0);
                if sampler_this != 0 {
                    gain_cap = 0.0;
                    for idx in 0..4u32 {
                        let d: f64 = callee_thiscall!(1, f64, sampler_this, idx);
                        let g = d as f32;
                        dist_term = g;
                        if !(gain_cap > g) {
                            gain_cap = g;
                        }
                    }
                    let t = gain_cap * 1.7000000476837158f32;
                    gain_cap = if t > 1.0 { 1.0 } else { t };
                    let mut slot = 0u32;
                    let vt = *(ent as *const u32);
                    let tgt = *((vt.wrapping_add(0xEC)) as *const u32);
                    let pos: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(tgt as usize);
                    let p = pos(ent, (&mut slot as *mut u32) as u32);
                    let px = *(p as *const f32);
                    let py = *((p.wrapping_add(4)) as *const f32);
                    let pz = *((p.wrapping_add(8)) as *const f32);
                    let n = fadd_first(fadd_first(px * px, py * py), pz * pz);
                    let t = n.sqrt() * 0.06666667014360428f32;
                    dist_term = if t > 1.0 { 1.0 } else { t };
                }
            } else if kind == 2 {
                let mut slot = 0u32;
                let vt = *(ent as *const u32);
                let tgt = *((vt.wrapping_add(0xEC)) as *const u32);
                let pos: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                let p = pos(ent, (&mut slot as *mut u32) as u32);
                let px = *(p as *const f32);
                let py = *((p.wrapping_add(4)) as *const f32);
                let pz = *((p.wrapping_add(8)) as *const f32);
                let n = fadd_first(fadd_first(px * px, py * py), pz * pz);
                let t = n.sqrt() * 0.05000000074505806f32;
                dist_term = if t > 1.0 { 1.0 } else { t };
            }
        }
        let ent2 = *(obj.wrapping_add(0x48) as *const u32);
        if ent2 != 0 && (*(ent2.wrapping_add(0x28) as *const u32) & 0x3C0) == 0x80 {
            let f54 = *(obj.wrapping_add(0x54) as *const f32);
            let _: u32 = callee_thiscall!(3, u32, this, ent2, f54.to_bits());
            return 0;
        }
        let f54 = *(obj.wrapping_add(0x54) as *const f32);
        let mut slot = 0u32;
        let ans: u32 = callee_thiscall!(4, u32, obj, (&mut slot as *mut u32) as u32);
        let level = fmul_first(dist_term, gain_cap);
        let _: u32 = callee_thiscall!(
            5, u32, this, (f54 * 0.5).to_bits(), level.to_bits(), ans
        );
        0
    }
});
