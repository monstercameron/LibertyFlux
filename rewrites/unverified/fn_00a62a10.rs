// original: 0x00a62a10 ped_task_pose_transform (proposed)

/// Transform two local-space vectors of a ped task object into world space.
///
/// `this` points to the task object: two 4-float vectors at `+0x00` and
/// `+0x10`, a dword at `+0x20` copied verbatim to `*out_c`, an optional
/// parent/link object pointer at `+0x24`, and a flag byte at `+0x28`.
///
/// Behaviour:
/// - If flag bit 0 is clear, return 0 and write nothing.
/// - If the link pointer is null, copy both vectors and the dword straight
///   through (`out_a = this[0..4]`, `out_b = this[4..8]`, `*out_c =
///   this[8]`), return 1.
/// - If the link is present but flag bit 1 is clear, copy the same way and
///   then fall through into the translation add below (the original has no
///   return between the copy and the tail), return 1.
/// - Otherwise (linked and bit 1 set): if the link's matrix pointer at
///   `+0x20` is null, call callee 0 (thiscall, `ecx` = link) to build it,
///   then callee 1 (thiscall, `ecx` = link + `0x10`, arg = matrix). Each
///   vector is then dotted with the matrix columns (`out[i] = col_i . v`,
///   columns at strides of `0x10`) into `out_a`/`out_b`; the `w` slots
///   receive an uninitialised stack word of the original (modelled as 0;
///   see below). Finally a translation is added element-wise
///   (`out[i] = T[i] + out[i]`), read from matrix `+0x30` when the matrix
///   is present, else from link `+0x10`. Return 1.
///
/// The `w` slot quirk: the original stores `[esp+0x1c]` there, a word its
/// own frame never wrote (below-ESP scratch). The checker runs both sides
/// with a defined scratch fill of 0, so the rewrite writes 0; any proof of
/// this function is narrower than the default by that word.
///
/// Float operation order is the original's SSE order, pinned through
/// `black_box` helpers. Original: 0x00a62a10 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00a62a10(this: u32, out_a: u32, out_b: u32, out_c: u32) -> u32 {
    unsafe {
        const VEC1: u32 = 0x00;
        const VEC2: u32 = 0x10;
        const TAG: u32 = 0x20;
        const LINK: u32 = 0x24;
        const FLAGS: u32 = 0x28;
        const LINK_MATRIX: u32 = 0x20;
        const LINK_FALLBACK: u32 = 0x10;
        const MATRIX_TRANS: u32 = 0x30;
        const FLAG_ACTIVE: u8 = 0x01;
        const FLAG_TRANSFORM: u8 = 0x02;
        const CALLEE_BUILD: u32 = 0;
        const CALLEE_FETCH: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        /// One column-major 3x3 transform: `dst[i] = col_i . v`, in the
        /// original's exact SSE order. `w_fill` stands in for the
        /// uninitialised word the original copies into the `w` slot.
        unsafe fn transform(mat: u32, vx: f32, vy: f32, vz: f32, dst: u32, w_fill: u32) {
            unsafe {
                let t0 = mul(rdf(mat), vx);
                let mut x = mul(rdf(mat + 0x10), vy);
                let mut y = mul(rdf(mat + 0x14), vy);
                x = add(x, t0);
                let t = mul(rdf(mat + 0x20), vz);
                let mut z = mul(rdf(mat + 0x18), vy);
                x = add(x, t);
                let t = mul(rdf(mat + 0x04), vx);
                y = add(y, t);
                let t = mul(rdf(mat + 0x24), vz);
                y = add(y, t);
                let t = mul(rdf(mat + 0x08), vx);
                z = add(z, t);
                let t = mul(rdf(mat + 0x28), vz);
                wrf(dst, x);
                wrf(dst + 4, y);
                z = add(z, t);
                wr32(dst + 0x0c, w_fill);
                wrf(dst + 8, z);
            }
        }

        let flags = rd8(this + FLAGS);
        if flags & FLAG_ACTIVE == 0 {
            return 0;
        }
        let link = rd32(this + LINK);
        if link == 0 {
            wr32(out_a, rd32(this + VEC1));
            wrf(out_a + 4, rdf(this + VEC1 + 4));
            wrf(out_a + 8, rdf(this + VEC1 + 8));
            wr32(out_a + 0x0c, rd32(this + VEC1 + 0x0c));
            wr32(out_b, rd32(this + VEC2));
            wrf(out_b + 4, rdf(this + VEC2 + 4));
            wrf(out_b + 8, rdf(this + VEC2 + 8));
            wr32(out_b + 0x0c, rd32(this + VEC2 + 0x0c));
            wr32(out_c, rd32(this + TAG));
            return 1;
        }
        if flags & FLAG_TRANSFORM == 0 {
            // Copy, then fall through to the translation tail like the original.
            wr32(out_a, rd32(this + VEC1));
            wrf(out_a + 4, rdf(this + VEC1 + 4));
            wrf(out_a + 8, rdf(this + VEC1 + 8));
            wr32(out_a + 0x0c, rd32(this + VEC1 + 0x0c));
            wr32(out_b, rd32(this + VEC2));
            wrf(out_b + 4, rdf(this + VEC2 + 4));
            wrf(out_b + 8, rdf(this + VEC2 + 8));
            wr32(out_b + 0x0c, rd32(this + VEC2 + 0x0c));
        } else {
            // Linked transform path; ensure the matrix exists first.
            if rd32(link + LINK_MATRIX) == 0 {
                lf_checker_rt::callee_thiscall!(CALLEE_BUILD, u32, link);
                let mat = rd32(link + LINK_MATRIX);
                lf_checker_rt::callee_thiscall!(CALLEE_FETCH, u32, link.wrapping_add(LINK_FALLBACK), mat);
            }
            let mat = rd32(link + LINK_MATRIX);
            transform(mat, rdf(this), rdf(this + 4), rdf(this + 8), out_a, 0);
            // Second site: the matrix is re-fetched (identical value) and the
            // build pair re-runs only if it somehow came back null.
            let link2 = rd32(this + LINK);
            if rd32(link2 + LINK_MATRIX) == 0 {
                lf_checker_rt::callee_thiscall!(CALLEE_BUILD, u32, link2);
                let base = link2.wrapping_add(LINK_FALLBACK);
                let mat2 = rd32(base + LINK_FALLBACK);
                lf_checker_rt::callee_thiscall!(CALLEE_FETCH, u32, base, mat2);
            }
            let mat = rd32(link2 + LINK_MATRIX);
            transform(mat, rdf(this + 0x10), rdf(this + 0x14), rdf(this + 0x18), out_b, 0);
        }
        // Translation add (matrix + 0x30, or link + 0x10 when null).
        let link3 = rd32(this + LINK);
        let m3 = rd32(link3 + LINK_MATRIX);
        let t = if m3 != 0 { m3.wrapping_add(MATRIX_TRANS) } else { link3.wrapping_add(LINK_FALLBACK) };
        wrf(out_a, add(rdf(t), rdf(out_a)));
        wrf(out_a + 4, add(rdf(t + 4), rdf(out_a + 4)));
        wrf(out_a + 8, add(rdf(t + 8), rdf(out_a + 8)));
        let link4 = rd32(this + LINK);
        let m4 = rd32(link4 + LINK_MATRIX);
        let t = if m4 != 0 { m4.wrapping_add(MATRIX_TRANS) } else { link4.wrapping_add(LINK_FALLBACK) };
        wrf(out_b, add(rdf(t), rdf(out_b)));
        wrf(out_b + 4, add(rdf(t + 4), rdf(out_b + 4)));
        wrf(out_b + 8, add(rdf(t + 8), rdf(out_b + 8)));
        wr32(out_c, rd32(this + TAG));
        1
    }
});
