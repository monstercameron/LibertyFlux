// original: 0x00c2ca80 audfire_ray_vs_sphere
// Shared helpers (identical in this lane's other rewrite files; keep one copy
// when merging): fadd_first/fmul_first reproduce one addss/mulss with
// first-operand NaN precedence, which Rust codegen does not otherwise pin.
#[inline(always)]
unsafe fn load_f(addr: u32) -> f32 {
    *(addr as *const f32)
}

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

/// Tests the ray (origin, dir) against the sphere (center at this+0, radius at
/// this+0x10), treating the direction as unit length. On a hit writes the near
/// hit point to out_near and the far hit point to out_far (x, y, z plus a w
/// component) and returns 1; on a miss writes nothing and returns 0.
///
/// The w component of each hit point is a copy of one uninitialized word of the
/// original's own stack frame, i.e. indeterminate garbage in production. Under
/// the checker's defined stack fill it is the fill value; the contract sets the
/// fill to zero so this rewrite stores 0.0 there. Only AL of the return value
/// is significant.
export!(thiscall, rw_00c2ca80(sphere: u32, origin: u32, dir: u32, out_near: u32, out_far: u32) -> u32 {
    unsafe {
        let cx = load_f(sphere);
        let cy = load_f(sphere + 4);
        let cz = load_f(sphere + 8);
        let radius = load_f(sphere + 0x10);
        let px = load_f(origin);
        let py = load_f(origin + 4);
        let pz = load_f(origin + 8);
        let dx = px - cx;
        let dy = py - cy;
        let dz = pz - cz;
        let dirx = load_f(dir);
        let diry = load_f(dir + 4);
        let dirz = load_f(dir + 8);
        // b = 2 * dot(dir, d), accumulated x, then y, then z.
        // Variable-variable commutative ops go through the NaN-explicit
        // helpers; self-products (x*x) and constant factors are order-safe.
        let mut b = fmul_first(dirx, dx);
        b = fadd_first(b, fmul_first(diry, dy));
        b = fadd_first(b, fmul_first(dirz, dz));
        b *= 2.0;
        // c = dot(d, d) - r^2, accumulated x, then y, then z.
        let mut c = dx * dx;
        c = fadd_first(c, dy * dy);
        c = fadd_first(c, dz * dz);
        c -= radius * radius;
        c *= 4.0;
        let disc = b * b - c;
        if disc < 0.0 {
            return 0;
        }
        let sq = disc.sqrt();
        // Near root first: t_near = (-b - sq) / 2, t_far = (sq - b) / 2.
        let t_near = (-b - sq) * 0.5;
        let t_far = (sq - b) * 0.5;
        // Near hit point. The direction components are reloaded from memory
        // because out_near may overlap the inputs.
        let ny = fadd_first(py, fmul_first(load_f(dir + 4), t_near));
        let nz = fadd_first(pz, fmul_first(load_f(dir + 8), t_near));
        let nx = fadd_first(px, fmul_first(load_f(dir), t_near));
        store_f(out_near + 4, ny);
        store_f(out_near + 8, nz);
        store_f(out_near + 0x0c, 0.0);
        store_f(out_near, nx);
        // Far hit point; origin and direction reloaded again for the same reason.
        let fx = fadd_first(load_f(origin), fmul_first(load_f(dir), t_far));
        let fy = fadd_first(fmul_first(load_f(dir + 4), t_far), load_f(origin + 4));
        let fz = fadd_first(load_f(origin + 8), fmul_first(t_far, load_f(dir + 8)));
        store_f(out_far + 4, fy);
        store_f(out_far, fx);
        store_f(out_far + 8, fz);
        store_f(out_far + 0x0c, 0.0);
        1
    }
});
