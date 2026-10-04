// original: 0x00A99BA0 merge_bounding_spheres
/// Merge two bounding spheres into their smallest shared sphere.
///
/// `a`/`b` point at two float quads (center xyz plus a carried tag word),
/// `v`/`u` are their radii, and the merged center/tag go to `out` with the
/// merged radius to `out_radius`.
///
/// When one sphere already contains the other (the radii differ by at least
/// the center distance), the larger sphere is copied through unchanged.
/// Otherwise the merged sphere has radius `(dist + u + v) / 2` with its
/// center on the segment between the inputs.
///
/// Quirk, reproduced faithfully: the merge path never writes a tag word and
/// instead copies one word of uninitialized stack to `out[3]`. The rewrite
/// reads its own unwritten stack slot the same way, so both sides observe
/// whatever the platform left there (the checker defines it as its fill).
export!(cdecl, rw_00a99ba0(a: u32, v: f32, b: u32, u: f32, out: u32, out_radius: u32) -> u32 {
    unsafe {
        let a0 = *(a as *const f32);
        let b0 = *(b as *const f32);
        let b1 = *((b + 4) as *const f32);
        let a1 = *((a + 4) as *const f32);
        let b2 = *((b + 8) as *const f32);
        let a2 = *((a + 8) as *const f32);
        let d0 = b0 - a0;
        let d1 = b1 - a1;
        let d2 = b2 - a2;
        // Squared center distance, accumulated in the original's exact
        // order: (d1*d1 + d0*d0) + d2*d2. Operand order matters for NaN
        // payloads and rustc commutes plain adds, so the right operand of
        // every order-sensitive add passes through black_box, which forces
        // it through memory and leaves the left operand as the add's
        // destination (verified in the emitted code).
        let s1 = d1 * d1;
        let s0 = d0 * d0;
        let s01 = s1 + core::hint::black_box(s0);
        let s2 = d2 * d2;
        let dist2 = s01 + core::hint::black_box(s2);
        let t = u - v;
        let t2 = t * t;
        // comiss+jb: branch when t2 < dist2 or either is NaN.
        if !(t2 >= dist2) {
            let dist = dist2.sqrt();
            let denom = dist * 2.0;
            let c = dist + core::hint::black_box(u);
            let k = (c - v) / denom;
            let r = (c + v) * 0.5;
            // The original copies an unwritten frame slot here; read an
            // unwritten stack slot the same way (volatile, so the read the
            // original performs is the read this performs).
            let slot = core::mem::MaybeUninit::<u32>::uninit();
            let tag = core::ptr::read_volatile(slot.as_ptr());
            // Store order mirrors the original (out[3] first) so any
            // caller-side aliasing between the outputs agrees too.
            *((out + 12) as *mut u32) = tag;
            *(out as *mut f32) = d0 * k + core::hint::black_box(a0);
            *((out + 4) as *mut f32) = d1 * k + core::hint::black_box(a1);
            *((out + 8) as *mut f32) = d2 * k + core::hint::black_box(a2);
            *(out_radius as *mut f32) = r;
        } else if !(t >= 0.0) {
            // Sphere A contains B: copy A through, radius v.
            *(out as *mut f32) = a0;
            *((out + 4) as *mut f32) = a1;
            *((out + 8) as *mut f32) = a2;
            *((out + 12) as *mut u32) = *((a + 12) as *const u32);
            *(out_radius as *mut f32) = v;
        } else {
            // Sphere B contains A: copy B through, radius u.
            *(out as *mut f32) = b0;
            *((out + 4) as *mut f32) = b1;
            *((out + 8) as *mut f32) = b2;
            *((out + 12) as *mut u32) = *((b + 12) as *const u32);
            *(out_radius as *mut f32) = u;
        }
        // The original leaves the radius-pointer argument in eax on exit.
        out_radius
    }
});
