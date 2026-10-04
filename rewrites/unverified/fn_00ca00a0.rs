// original: 0x00CA00A0 ped_task_pose_combine (proposed)

/// Combine a task pose matrix with one row of a per-index table.
///
/// `this` points to a task object whose word at `+0x40` is the inner
/// worker object. Two resolver calls (one per stack argument, callee id 1)
/// fetch the input matrix `a` (from `tag`) and the output matrix `out`
/// (from `index`). A virtual check (slot `+0xa0` on the inner object) then
/// picks the table: when it returns non-zero the table comes from a second
/// check call followed by a virtual fetch (slot `+0xe0`); otherwise it
/// comes from the pointer at inner `+0x100`. Either way the table pointer
/// is the word at `+4`, and the row base is the word at its `+0x10` plus
/// `index * 64`.
///
/// The kernel writes twelve floats: `out` is the 3x4 product of the 3x3
/// part of `a` (rows at `+0x0`, `+0x10`, `+0x20`) with the four 3-float
/// columns of the row (at `+0x0`, `+0x10`, `+0x20`, `+0x30`), the last
/// column additionally translated by `a` at `+0x30`, `+0x34`, `+0x38`.
/// Each output is a sum of products in the original's operand order (the
/// middle row of each column starts with the `+0x14`/`+0x18` term, and the
/// translated column starts with the `+0x4` term); words `+0xc`, `+0x1c`
/// and `+0x2c` of `out` are left untouched. No branch depends on a float.
///
/// Original: 0x00CA00A0 (thiscall, ecx = this, two stack words; returns
/// the output pointer in eax).
lf_checker_rt::export!(thiscall, rw_00CA00A0(this: u32, index: u32, tag: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x40;
        const VTABLE_CHECK: u32 = 0xa0;
        const VTABLE_FETCH: u32 = 0xe0;
        const ALT_TABLE: u32 = 0x100;
        const ROW_STRIDE: u32 = 64;
        const RESOLVER: u32 = 1;

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

        let inner = rd32(this + INNER);
        let a = lf_checker_rt::callee_thiscall!(RESOLVER, u32, inner, tag);
        let out = lf_checker_rt::callee_thiscall!(RESOLVER, u32, inner, index);
        let table = {
            let check: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(inner) + VTABLE_CHECK) as usize);
            if check(inner) != 0 {
                let again: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(inner) + VTABLE_CHECK) as usize);
                let fetched = again(inner);
                let fetch: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(fetched) + VTABLE_FETCH) as usize);
                rd32(fetch(fetched) + 4)
            } else {
                rd32(rd32(inner + ALT_TABLE) + 4)
            }
        };
        let row = rd32(table + 0x10).wrapping_add(index.wrapping_mul(ROW_STRIDE));

        // Column 0: out[0..2] = A rows dotted with row[0..2].
        let p00 = mul(rdf(a + 0x00), rdf(row + 0x00));
        let p01 = mul(rdf(a + 0x10), rdf(row + 0x04));
        let p02 = mul(rdf(a + 0x20), rdf(row + 0x08));
        wrf(out + 0x00, add(add(p00, p01), p02));
        let q00 = mul(rdf(a + 0x14), rdf(row + 0x04));
        let q01 = mul(rdf(a + 0x04), rdf(row + 0x00));
        let q02 = mul(rdf(a + 0x24), rdf(row + 0x08));
        wrf(out + 0x04, add(add(q00, q01), q02));
        let r00 = mul(rdf(a + 0x18), rdf(row + 0x04));
        let r01 = mul(rdf(a + 0x08), rdf(row + 0x00));
        let r02 = mul(rdf(a + 0x28), rdf(row + 0x08));
        wrf(out + 0x08, add(add(r00, r01), r02));
        // Column 1: same rows dotted with row[0x10..0x18].
        let p10 = mul(rdf(a + 0x00), rdf(row + 0x10));
        let p11 = mul(rdf(a + 0x10), rdf(row + 0x14));
        let p12 = mul(rdf(a + 0x20), rdf(row + 0x18));
        wrf(out + 0x10, add(add(p10, p11), p12));
        let q10 = mul(rdf(a + 0x14), rdf(row + 0x14));
        let q11 = mul(rdf(a + 0x04), rdf(row + 0x10));
        let q12 = mul(rdf(a + 0x24), rdf(row + 0x18));
        wrf(out + 0x14, add(add(q10, q11), q12));
        let r10 = mul(rdf(a + 0x18), rdf(row + 0x14));
        let r11 = mul(rdf(a + 0x08), rdf(row + 0x10));
        let r12 = mul(rdf(a + 0x28), rdf(row + 0x18));
        wrf(out + 0x18, add(add(r10, r11), r12));
        // Column 2: same rows dotted with row[0x20..0x28].
        let p20 = mul(rdf(a + 0x00), rdf(row + 0x20));
        let p21 = mul(rdf(a + 0x10), rdf(row + 0x24));
        let p22 = mul(rdf(a + 0x20), rdf(row + 0x28));
        wrf(out + 0x20, add(add(p20, p21), p22));
        let q20 = mul(rdf(a + 0x14), rdf(row + 0x24));
        let q21 = mul(rdf(a + 0x04), rdf(row + 0x20));
        let q22 = mul(rdf(a + 0x24), rdf(row + 0x28));
        wrf(out + 0x24, add(add(q20, q21), q22));
        let r20 = mul(rdf(a + 0x18), rdf(row + 0x24));
        let r21 = mul(rdf(a + 0x08), rdf(row + 0x20));
        let r22 = mul(rdf(a + 0x28), rdf(row + 0x28));
        wrf(out + 0x28, add(add(r20, r21), r22));
        // Column 3: dotted with row[0x30..0x38], then translated.
        let p30 = mul(rdf(a + 0x04), rdf(row + 0x30));
        let p31 = mul(rdf(a + 0x14), rdf(row + 0x34));
        let p32 = mul(rdf(a + 0x24), rdf(row + 0x38));
        wrf(out + 0x34, add(add(add(p30, p31), p32), rdf(a + 0x34)));
        let q30 = mul(rdf(a + 0x00), rdf(row + 0x30));
        let q31 = mul(rdf(a + 0x10), rdf(row + 0x34));
        let q32 = mul(rdf(a + 0x20), rdf(row + 0x38));
        wrf(out + 0x30, add(add(add(q30, q31), q32), rdf(a + 0x30)));
        let r30 = mul(rdf(a + 0x08), rdf(row + 0x30));
        let r31 = mul(rdf(a + 0x18), rdf(row + 0x34));
        let r32 = mul(rdf(a + 0x28), rdf(row + 0x38));
        wrf(out + 0x38, add(add(add(r30, r31), r32), rdf(a + 0x38)));
        out
    }
});
