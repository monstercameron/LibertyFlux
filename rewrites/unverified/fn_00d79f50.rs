// original: 0x00d79f50 scene_xform_dispatch (proposed)

/// Dispatch a scene-graph transform by the object's kind byte (+0x04).
///
/// `obj` points to the object, `mtx` to the parent 3x4 matrix (columns at
/// +0x00/+0x10/+0x20, +0x04/+0x14/+0x24, +0x08/+0x18/+0x28, translation at
/// +0x30/+0x34/+0x38); `a1`..`a3` are passed through to the callees untouched.
///
/// Kind 12 walks the child-pointer array (+0x80) `count` times (`count` is
/// the u16 at +0x92; entry exits when it is 0, the loop index is compared
/// SIGNED), composing each non-null child's local matrix (+0x84, stride
/// 0x40) with the parent matrix and recursing on the child; it returns
/// `count`. Kind 6 runs `count` elements (the i32 at +0xcc, must be > 0,
/// compared SIGNED) through the resolve/decode/transform/combine chain and
/// returns the last combine answer. Kinds 3 and 4 run the same chain with
/// indices stored in each element and the entry count as the SIGNED trip
/// bound, returning the trip count. Kind 0 transforms the vector at +0x30,
/// calls virtual slot
/// +0x44 and returns the finish callee's answer. Any other kind returns the
/// leftover `(a3 with its low byte replaced by the kind)` and does nothing.
///
/// All float arithmetic is the original's exact operation and operand order
/// (pinned against reassociation); the first composed row deliberately
/// differs in term order from the others. Original: cdecl, five stack words.
/// Compared values: `count`/`bound` SIGNED everywhere except the kind-12
/// entry check, which exits on `count == 0` (unsigned `jae`).
lf_checker_rt::export!(cdecl, rw_00d79f50(obj: u32, a1: u32, a2: u32, a3: u32, mtx: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        // Object layout (all offsets from the object pointer `obj`).
        const OBJ_KIND: u32 = 0x04;
        const OBJ_VTABLE: u32 = 0x00;
        const OBJ_VEC: u32 = 0x30; // 3 floats (kind 0)
        const OBJ_CHILDREN: u32 = 0x80; // child-pointer array (kind 12)
        const OBJ_LOCAL_MATS: u32 = 0x84; // local 3x4 matrices, stride 0x40 (kind 12)
        const OBJ_ELEMS: u32 = 0x8c; // element array (kinds 6, 3, 4)
        const OBJ_COUNT16: u32 = 0x92; // u16 trip count (kind 12)
        const OBJ_TABLE: u32 = 0xb0; // packed i16 triple table (kinds 6, 3, 4)
        const OBJ_COUNT: u32 = 0xcc; // i32 trip count / entry gate (kinds 6, 3, 4)
        // Scale/offset floats for packed-triple decode.
        const OBJ_SX: u32 = 0x90;
        const OBJ_SY: u32 = 0x94;
        const OBJ_SZ: u32 = 0x98;
        const OBJ_BX: u32 = 0xa0;
        const OBJ_BY: u32 = 0xa4;
        const OBJ_BZ: u32 = 0xa8;
        // Parent matrix `mtx`: columns at +0x00/+0x10/+0x20, +0x04/+0x14/+0x24,
        // +0x08/+0x18/+0x28, translation at +0x30/+0x34/+0x38.
        const VT_SLOT_XFORM: u32 = 0x44;

        // Callee ids (see contract).
        const C_RECURSE: u32 = 1;
        const C_RESOLVE: u32 = 2;
        const C_XFORM_POINT: u32 = 3;
        const C_COMBINE: u32 = 4;
        const C_VT_XFORM: u32 = 5;
        const C_FINISH: u32 = 6;
        const C_COOKIE: u32 = 7;

        /// Decode one packed triple: index -> three i16 -> scaled floats.
        /// `idx` is an unsigned 16-bit table index; the table holds 6-byte entries.
        /// Order of operations is the original's: (w*s)+b per lane.
        #[inline(always)]
        unsafe fn decode_triple(table: u32, idx: u32, obj: u32) -> (f32, f32, f32) {
            unsafe {
                let t = idx.wrapping_mul(3);
                let base = table.wrapping_add(t.wrapping_mul(2));
                let w0 = (rd16(base) as u16 as i16) as f32;
                let w1 = (rd16(base.wrapping_add(2)) as u16 as i16) as f32;
                let w2 = (rd16(base.wrapping_add(4)) as u16 as i16) as f32;
                let x = add(mul(w0, rdf(obj.wrapping_add(OBJ_SX))), rdf(obj.wrapping_add(OBJ_BX)));
                let y = add(mul(w1, rdf(obj.wrapping_add(OBJ_SY))), rdf(obj.wrapping_add(OBJ_BY)));
                let z = add(mul(w2, rdf(obj.wrapping_add(OBJ_SZ))), rdf(obj.wrapping_add(OBJ_BZ)));
                (x, y, z)
            }
        }

        /// Kind 12: walk the child array, compose each non-null child's local matrix
        /// with the parent matrix, recurse on the child. `count` (u16 at +0x92) is
        /// compared unsigned at entry (exit when 0) and the loop index against it is
        /// compared SIGNED (jl); both operands stay small. Returns `count`.
        unsafe fn branch_compose_children(obj: u32, a1: u32, a2: u32, a3: u32, mtx: u32) -> u32 {
            unsafe {
                let count = rd16(obj.wrapping_add(OBJ_COUNT16));
                if count == 0 {
                    return 0;
                }
                let children = rd32(obj.wrapping_add(OBJ_CHILDREN));
                let mats = rd32(obj.wrapping_add(OBJ_LOCAL_MATS));
                let m00 = rdf(mtx.wrapping_add(0x00));
                let m01 = rdf(mtx.wrapping_add(0x04));
                let m02 = rdf(mtx.wrapping_add(0x08));
                let m10 = rdf(mtx.wrapping_add(0x10));
                let m11 = rdf(mtx.wrapping_add(0x14));
                let m12 = rdf(mtx.wrapping_add(0x18));
                let m20 = rdf(mtx.wrapping_add(0x20));
                let m21 = rdf(mtx.wrapping_add(0x24));
                let m22 = rdf(mtx.wrapping_add(0x28));
                let t0 = rdf(mtx.wrapping_add(0x30));
                let t1 = rdf(mtx.wrapping_add(0x34));
                let t2 = rdf(mtx.wrapping_add(0x38));
                let mut i: u32 = 0;
                let mut off: u32 = 0;
                loop {
                    let child = rd32(children.wrapping_add(i.wrapping_mul(4)));
                    if child != 0 {
                        let a = mats.wrapping_add(off);
                        let a00 = rdf(a.wrapping_add(0x00));
                        let a01 = rdf(a.wrapping_add(0x04));
                        let a02 = rdf(a.wrapping_add(0x08));
                        let a10 = rdf(a.wrapping_add(0x10));
                        let a11 = rdf(a.wrapping_add(0x14));
                        let a12 = rdf(a.wrapping_add(0x18));
                        let a20 = rdf(a.wrapping_add(0x20));
                        let a21 = rdf(a.wrapping_add(0x24));
                        let a22 = rdf(a.wrapping_add(0x28));
                        let a30 = rdf(a.wrapping_add(0x30));
                        let a31 = rdf(a.wrapping_add(0x34));
                        let a32 = rdf(a.wrapping_add(0x38));
                        // Row 0 of the composed matrix. Note the first two terms are
                        // in the opposite order from every other row: the original
                        // loads the +4 element first here only.
                        let o0 = add(add(mul(a01, m10), mul(a00, m00)), mul(m20, a02));
                        let o1 = add(add(mul(a01, m11), mul(a00, m01)), mul(m21, a02));
                        let o2 = add(add(mul(a01, m12), mul(a00, m02)), mul(m22, a02));
                        let o3 = add(add(mul(a10, m00), mul(a11, m10)), mul(m20, a12));
                        let o4 = add(add(mul(a10, m01), mul(a11, m11)), mul(m21, a12));
                        let o5 = add(add(mul(a10, m02), mul(a11, m12)), mul(m22, a12));
                        let o6 = add(add(mul(a20, m00), mul(a21, m10)), mul(m20, a22));
                        let o7 = add(add(mul(a20, m01), mul(a21, m11)), mul(m21, a22));
                        let o8 = add(add(mul(a20, m02), mul(a21, m12)), mul(m22, a22));
                        let o9 = add(add(add(mul(a30, m00), mul(a31, m10)), mul(m20, a32)), t0);
                        let o10 = add(add(add(mul(a30, m01), mul(a31, m11)), mul(m21, a32)), t1);
                        let o11 = add(add(add(mul(a30, m02), mul(a31, m12)), mul(m22, a32)), t2);
                        // 4 rows on a 16-byte stride; slots 3, 7, 11 are never
                        // stored by the original (uninitialized stack there).
                        let mut mat = [0u32; 16];
                        mat[0] = o0.to_bits();
                        mat[1] = o1.to_bits();
                        mat[2] = o2.to_bits();
                        mat[4] = o3.to_bits();
                        mat[5] = o4.to_bits();
                        mat[6] = o5.to_bits();
                        mat[8] = o6.to_bits();
                        mat[9] = o7.to_bits();
                        mat[10] = o8.to_bits();
                        mat[12] = o9.to_bits();
                        mat[13] = o10.to_bits();
                        mat[14] = o11.to_bits();
                        lf_checker_rt::callee_cdecl!(C_RECURSE, u32, child, a1, a2, a3, mat.as_ptr() as u32);
                    }
                    i = i.wrapping_add(1);
                    off = off.wrapping_add(0x40);
                    if !((i as i32) < (count as i32)) {
                        break;
                    }
                }
                count
            }
        }

        /// Run one element's three decode/transform/combine step shared by kinds 6, 3
        /// and 4. Returns the combine callee's answer.
        unsafe fn combine_step(
            obj: u32, mtx: u32, a1: u32, a2: u32, a3: u32, idx0: u32, idx1: u32, idx2: u32,
        ) -> u32 {
            unsafe {
                let table = rd32(obj.wrapping_add(OBJ_TABLE));
                let (x0, y0, z0) = decode_triple(table, idx0, obj);
                let (x1, y1, z1) = decode_triple(table, idx1, obj);
                let (x2, y2, z2) = decode_triple(table, idx2, obj);
                let input0 = [x0.to_bits(), y0.to_bits(), z0.to_bits()];
                let input1 = [x1.to_bits(), y1.to_bits(), z1.to_bits()];
                let input2 = [x2.to_bits(), y2.to_bits(), z2.to_bits()];
                let mut out0 = [0u32; 8];
                let mut out1 = [0u32; 8];
                let mut out2 = [0u32; 8];
                lf_checker_rt::callee_cdecl!(C_XFORM_POINT, u32, out0.as_mut_ptr() as u32, mtx, input0.as_ptr() as u32);
                lf_checker_rt::callee_cdecl!(C_XFORM_POINT, u32, out1.as_mut_ptr() as u32, mtx, input1.as_ptr() as u32);
                lf_checker_rt::callee_cdecl!(C_XFORM_POINT, u32, out2.as_mut_ptr() as u32, mtx, input2.as_ptr() as u32);
                // The combine call reads the first two outputs one word in and the
                // third from its start, exactly as the original's frame offsets.
                lf_checker_rt::callee_cdecl!(
                    C_COMBINE, u32,
                    out0.as_ptr().add(1) as u32,
                    out1.as_ptr().add(1) as u32,
                    out2.as_ptr() as u32,
                    a1, a2, a3
                )
            }
        }

        /// Kind 6: for each of `count` (i32 at +0xcc, must be > 0) elements, resolve
        /// three table indices through the resolve callee, decode, transform and
        /// combine. Returns the last combine answer.
        unsafe fn branch_resolve_loop(obj: u32, a1: u32, a2: u32, a3: u32, mtx: u32) -> u32 {
            unsafe {
                let count = rd32(obj.wrapping_add(OBJ_COUNT)) as i32;
                if count <= 0 {
                    return count as u32;
                }
                let base = rd32(obj.wrapping_add(OBJ_ELEMS));
                let mut cursor: u32 = 0;
                let mut rem = count;
                let mut ans: u32 = 0;
                loop {
                    let elem = base.wrapping_add(cursor);
                    let mut frame = [0u32; 8];
                    lf_checker_rt::callee_thiscall!(C_RESOLVE, u32, frame.as_mut_ptr() as u32, elem);
                    let w0 = frame[4];
                    let w1 = frame[5];
                    let idx0 = w0 & 0xffff;
                    let idx1 = (w0 >> 16) & 0xffff;
                    let idx2 = w1 & 0xffff;
                    ans = combine_step(obj, mtx, a1, a2, a3, idx0, idx1, idx2);
                    cursor = cursor.wrapping_add(0x20);
                    rem -= 1;
                    if rem == 0 {
                        break;
                    }
                }
                ans
            }
        }

        /// Kinds 3 and 4: like kind 6, but the three indices come from the element
        /// itself (+0x10/+0x12/+0x14) and the trip bound is the entry count (the
        /// i32 at +0xcc, must be > 0), compared SIGNED (jl) against the 1-based
        /// loop index. Returns the trip count (the index is reloaded into eax).
        unsafe fn branch_chained_loop(obj: u32, a1: u32, a2: u32, a3: u32, mtx: u32) -> u32 {
            unsafe {
                let bound = rd32(obj.wrapping_add(OBJ_COUNT)) as i32;
                if bound <= 0 {
                    return (a3 & 0xffff_ff00) | (rd8(obj.wrapping_add(OBJ_KIND)) as u32);
                }
                let base = rd32(obj.wrapping_add(OBJ_ELEMS));
                let mut i: i32 = 0;
                let mut cursor: u32 = 0;
                loop {
                    let esi = base.wrapping_add(cursor);
                    let idx0 = rd16(esi.wrapping_add(0x10));
                    let idx1 = rd16(esi.wrapping_add(0x12));
                    let idx2 = rd16(esi.wrapping_add(0x14));
                    combine_step(obj, mtx, a1, a2, a3, idx0, idx1, idx2);
                    i = i.wrapping_add(1);
                    cursor = cursor.wrapping_add(0x20);
                    if !(i < bound) {
                        break;
                    }
                }
                // The original reloads the loop index into eax after the last
                // combine call, so it returns the trip count, not the answer.
                i as u32
            }
        }

        /// Kind 0: transform the vector at +0x30 by the parent matrix, hand the
        /// result to the object's virtual slot +0x44, then finish. Returns the finish
        /// callee's answer.
        unsafe fn branch_virtual(obj: u32, a1: u32, a2: u32, a3: u32, mtx: u32) -> u32 {
            unsafe {
                let v0 = rdf(obj.wrapping_add(OBJ_VEC));
                let v1 = rdf(obj.wrapping_add(OBJ_VEC).wrapping_add(4));
                let v2 = rdf(obj.wrapping_add(OBJ_VEC).wrapping_add(8));
                let m00 = rdf(mtx.wrapping_add(0x00));
                let m01 = rdf(mtx.wrapping_add(0x04));
                let m02 = rdf(mtx.wrapping_add(0x08));
                let m10 = rdf(mtx.wrapping_add(0x10));
                let m11 = rdf(mtx.wrapping_add(0x14));
                let m12 = rdf(mtx.wrapping_add(0x18));
                let m20 = rdf(mtx.wrapping_add(0x20));
                let m21 = rdf(mtx.wrapping_add(0x24));
                let m22 = rdf(mtx.wrapping_add(0x28));
                let t0 = rdf(mtx.wrapping_add(0x30));
                let t1 = rdf(mtx.wrapping_add(0x34));
                let t2 = rdf(mtx.wrapping_add(0x38));
                // Exact operation and operand order of the original.
                let o0 = add(add(add(mul(m10, v1), mul(v0, m00)), mul(m20, v2)), t0);
                let o1 = add(add(add(mul(m11, v1), mul(m01, v0)), mul(m21, v2)), t1);
                let o2 = add(add(add(mul(m12, v1), mul(m02, v0)), mul(m22, v2)), t2);
                // Word 3 is uninitialized stack in the original; it is never
                // snapshotted, so the value stored here is unobserved.
                let frame = [o0.to_bits(), o1.to_bits(), o2.to_bits(), 0u32];
                let vt = rd32(obj.wrapping_add(OBJ_VTABLE));
                let target = rd32(vt.wrapping_add(VT_SLOT_XFORM));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(target as usize) };
                f(obj, frame.as_ptr() as u32);
                lf_checker_rt::callee_cdecl!(C_FINISH, u32, obj, a1, a2, a3)
            }
        }
                let kind = rd8(obj.wrapping_add(OBJ_KIND));
                let r = match kind {
                    12 => branch_compose_children(obj, a1, a2, a3, mtx),
                    6 => branch_resolve_loop(obj, a1, a2, a3, mtx),
                    3 | 4 => branch_chained_loop(obj, a1, a2, a3, mtx),
                    0 => branch_virtual(obj, a1, a2, a3, mtx),
                    _ => (a3 & 0xffff_ff00) | (kind as u32),
                };
                // The original checks its stack cookie on every exit; the stub
                // preserves all registers, so this only records the call.
                lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
                r
    }
});
