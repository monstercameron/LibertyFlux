// original: 0x00C8F710 task_pose_query (proposed)

/// Query the pose contribution of one task node into a three-float buffer.
///
/// `this` points to the node: tag word at `+0x00` (low 3 bits select the
/// behaviour), a fallback position at `+0x04..+0x0c`, and a child pointer at
/// `+0x10`. `out` receives three floats. The node first copies its fallback
/// position to `out`, then returns unless the tag is 2 or 3 and the child is
/// non-null (returning `tag - 2` on those early paths).
///
/// On the main path a sibling query fills three floats through an out-pointer,
/// the child's matrix at `+0x20` is created on demand (a zero slot triggers an
/// init call followed by a link call taking the fresh slot value), and `out`
/// becomes the 3x3 matrix-times-vector product of the rows at `+0x00..+0x08`,
/// `+0x10..+0x18`, `+0x20..+0x28` with the queried point minus the translation
/// at `+0x30..+0x38`. Each row accumulates as `(m1*dy + m0*dx) + m2*dz` in the
/// original's operand order. Returns the matrix pointer.
///
/// Original: 0x00C8F710 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00C8F710(this: u32, out: u32) -> u32 {
    unsafe {
        const TAG_BITS: u32 = 7;
        const OFF_POS: u32 = 0x04;
        const OFF_CHILD: u32 = 0x10;
        const OFF_MAT: u32 = 0x20;
        const OFF_TX: u32 = 0x30;
        const OFF_TY: u32 = 0x34;
        const OFF_TZ: u32 = 0x38;
        const CAL_SIBLING: u32 = 1;
        const CAL_INIT: u32 = 2;
        const CAL_LINK: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        wr32(out, rd32(this + OFF_POS));
        wr32(out + 0x04, rd32(this + OFF_POS + 0x04));
        wr32(out + 0x08, rd32(this + OFF_POS + 0x08));
        let sel = (rd32(this) & TAG_BITS).wrapping_sub(2);
        if sel > 1 {
            return sel;
        }
        if rd32(this + OFF_CHILD) == 0 {
            return sel;
        }
        let mut point = [0u32; 3];
        lf_checker_rt::callee_thiscall!(CAL_SIBLING, u32, this, point.as_mut_ptr() as u32, 0);
        let child = rd32(this + OFF_CHILD);
        if rd32(child + OFF_MAT) == 0 {
            lf_checker_rt::callee_thiscall!(CAL_INIT, u32, child);
            let fresh = rd32(child + OFF_MAT);
            lf_checker_rt::callee_thiscall!(CAL_LINK, u32, child.wrapping_add(OFF_CHILD), fresh);
        }
        let mat = rd32(child + OFF_MAT);
        let dx = sub(f32::from_bits(point[0]), rdf(mat + OFF_TX));
        let dy = sub(f32::from_bits(point[1]), rdf(mat + OFF_TY));
        let dz = sub(f32::from_bits(point[2]), rdf(mat + OFF_TZ));
        let r0 = add(add(mul(rdf(mat + 0x04), dy), mul(rdf(mat), dx)), mul(rdf(mat + 0x08), dz));
        let r1 = add(
            add(mul(rdf(mat + 0x14), dy), mul(rdf(mat + 0x10), dx)),
            mul(rdf(mat + 0x18), dz),
        );
        let r2 = add(
            add(mul(rdf(mat + 0x24), dy), mul(rdf(mat + 0x20), dx)),
            mul(rdf(mat + 0x28), dz),
        );
        wrf(out, r0);
        wrf(out + 0x04, r1);
        wrf(out + 0x08, r2);
        mat
    }
});
