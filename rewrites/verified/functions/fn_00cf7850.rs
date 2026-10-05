// original: 0x00cf7850 ladder_mount_points_world (proposed)

/// Resolve one task node by type and index, copy its three anchor points,
/// and transform them into world space.
///
/// `obj` points to the task owner: a signed 16-bit type id at `+0x2E`
/// selects a table entry from the type table, a lazily built 4x3 matrix
/// lives at `+0x20` (position and heading at `+0x10` feed its builder),
/// and the three outputs are float4 slots. `index` is forwarded to the
/// node lookup. `out_a` receives the anchor at node `+0x1C`, `out_b` the
/// one at `+0x10`, `out_c` the one at `+0x28`; `out_flag` receives the
/// node's flag byte at `+0x34`.
///
/// Behaviour: look the entry up, fetch the node, and ask it for its kind
/// through its second virtual slot; anything but kind `0x0E` returns 0
/// with the kind's high bytes kept. Otherwise fetch the anchor source
/// through the virtual slot at `+0x44`, copy the three float3 anchors,
/// build the matrix when the cached pointer is null (allocate, then fill
/// from position and heading), and transform each anchor by the matrix
/// (`out = M * v + t`, in the original's operand order), except the third
/// anchor, whose block never reads the translation row (`out = M * v`).
/// The fourth component of each output is NOT computed: the original
/// stores a dword from its own frame that nothing ever writes
/// (uninitialized stack, the same slot for all three). Under the
/// checker's defined zero stack fill that dword is 0, so this rewrite
/// stores `0.0`; in the game the three w components are indeterminate
/// and their readers must ignore them. Returns `(out_flag & ~0xFF) | 1`.
///
/// Original: 0x00CF7850 (cdecl/6; reads no registers on entry).
lf_checker_rt::export!(cdecl, rw_00cf7850(obj: u32, index: u32, out_b: u32, out_a: u32, out_c: u32, out_flag: u32) -> u32 {
    unsafe {
        const TYPE_ID: u32 = 0x2E;
        const MATRIX_PTR: u32 = 0x20;
        const POS_HEADING: u32 = 0x10;
        const TYPE_TABLE: u32 = 0x1295CD8;
        const KIND_OK: u32 = 0x0E;
        const VF_KIND: u32 = 4;
        const VF_SRC: u32 = 0x44;
        const SRC_B: u32 = 0x10;
        const SRC_A: u32 = 0x1C;
        const SRC_C: u32 = 0x28;
        const SRC_FLAG: u32 = 0x34;
        const CAL_LOOKUP: u32 = 0;
        const CAL_KIND: u32 = 1;
        const CAL_SRC: u32 = 2;
        const CAL_ALLOC: u32 = 3;
        const CAL_BUILD: u32 = 4;
        /// Value of the uninitialized frame slot under `stack_fill: 0`
        /// (see the doc comment: the original reads it, nothing writes it).
        const UNINIT_W: f32 = 0.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        /// Transform one anchor by the matrix, in the original's order:
        /// x = (m4*v1 + m0*v0) + m8*v2 + t0 (and the y/z analogues).
        /// When `translate` is false the translation adds are skipped: the
        /// original's third block never reads the translation row.
        unsafe fn xform(out: u32, m: u32, translate: bool) {
            unsafe {
                let v0 = rdf(out);
                let v1 = rdf(out.wrapping_add(4));
                let v2 = rdf(out.wrapping_add(8));
                let m0 = rdf(m);
                let m1 = rdf(m.wrapping_add(4));
                let m2 = rdf(m.wrapping_add(8));
                let m4 = rdf(m.wrapping_add(0x10));
                let m5 = rdf(m.wrapping_add(0x14));
                let m6 = rdf(m.wrapping_add(0x18));
                let m8 = rdf(m.wrapping_add(0x20));
                let m9 = rdf(m.wrapping_add(0x24));
                let m10 = rdf(m.wrapping_add(0x28));
                let p_m0v0 = mul(m0, v0);
                let p_m4v1 = mul(m4, v1);
                let p_v1m5 = mul(v1, m5);
                let mut x = add(p_m4v1, p_m0v0);
                x = add(x, mul(m8, v2));
                let mut y = add(p_v1m5, mul(v0, m1));
                y = add(y, mul(m9, v2));
                let mut z = add(mul(m6, v1), mul(m2, v0));
                z = add(z, mul(m10, v2));
                if translate {
                    x = add(x, rdf(m.wrapping_add(0x30)));
                    y = add(y, rdf(m.wrapping_add(0x34)));
                    z = add(z, rdf(m.wrapping_add(0x38)));
                }
                wrf(out, x);
                wrf(out.wrapping_add(4), y);
                wrf(out.wrapping_add(8), z);
                wrf(out.wrapping_add(0x0C), UNINIT_W);
            }
        }

        // Signed type id selects the table entry (wrapping address math,
        // as the indexed addressing mode does).
        let type_id = ((obj.wrapping_add(TYPE_ID)) as *const i16).read_unaligned() as i32;
        let entry = (lf_checker_rt::relocated(TYPE_TABLE)
            .wrapping_add((type_id as u32).wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let node: u32 = lf_checker_rt::callee_thiscall!(CAL_LOOKUP, u32, entry, index);
        // Kind check through the second virtual slot.
        let vtbl = (node as *const u32).read_unaligned();
        let f_kind: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtbl.wrapping_add(VF_KIND)) as usize);
        let kind = f_kind(node);
        let _ = CAL_KIND;
        if kind & 0xFF != KIND_OK {
            return kind & 0xFFFFFF00;
        }
        // Anchor source through the virtual slot at +0x44.
        let f_src: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtbl.wrapping_add(VF_SRC)) as usize);
        let src = f_src(node);
        let _ = CAL_SRC;
        // Copy the three float3 anchors, one vector at a time (the
        // original's order, kept so overlapping outputs match too).
        for i in 0..3u32 {
            wrf(
                out_a.wrapping_add(i.wrapping_mul(4)),
                rdf(src.wrapping_add(SRC_A).wrapping_add(i.wrapping_mul(4))),
            );
        }
        for i in 0..3u32 {
            wrf(
                out_b.wrapping_add(i.wrapping_mul(4)),
                rdf(src.wrapping_add(SRC_B).wrapping_add(i.wrapping_mul(4))),
            );
        }
        for i in 0..3u32 {
            wrf(
                out_c.wrapping_add(i.wrapping_mul(4)),
                rdf(src.wrapping_add(SRC_C).wrapping_add(i.wrapping_mul(4))),
            );
        }
        // Lazily build the matrix (the fill call reads the pointer the
        // allocate call just stored, in this order).
        if rd32(obj.wrapping_add(MATRIX_PTR)) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_ALLOC, u32, obj);
            let m_now = rd32(obj.wrapping_add(MATRIX_PTR));
            let _: u32 =
                lf_checker_rt::callee_thiscall!(CAL_BUILD, u32, obj.wrapping_add(POS_HEADING), m_now);
        }
        let m = rd32(obj.wrapping_add(MATRIX_PTR));
        xform(out_a, m, true);
        xform(out_b, m, true);
        xform(out_c, m, false);
        ((out_flag as *mut u8)).write(((src.wrapping_add(SRC_FLAG)) as *const u8).read());
        (out_flag & 0xFFFFFF00) | 1
    }
});
