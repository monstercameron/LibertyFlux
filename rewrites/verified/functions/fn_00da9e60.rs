// original: 0x00DA9E60 ped_task_table_scan (proposed)

/// Scan a task table for the first entry whose probe window passes.
///
/// Arguments (cdecl, two stack words): `arg0` (opaque handle forwarded to
/// the resolver call), `arg1` (pointer to a task record; word at `+0`
/// selects the entry, words at `+0x10` select the window).
///
/// Behaviour: resolves the entry through callee 0; when that yields null
/// or an entry whose 16-bit tag at `+0x2E` is 0xFFFF, resolves through
/// callee 1 instead (with a scratch buffer and a zero flag). A null entry
/// or a 0xFFFF tag returns 0. Otherwise indexes a global object table by
/// the tag; a null slot, or an object without flag bit 0x10000 at `+0x40`,
/// returns 0. Then asks the object for its item count twice (callee 2);
/// either answer at or below zero returns 0. For each index below the
/// count: fetches the item (callee 3), keeps only items whose type query
/// (virtual slot `+4`) answers 0x0E, reads a 6-float parameter block at
/// `+0x10`..`+0x24` from the item's matrix object (virtual slot `+0x44`),
/// ensures the entry's 0x3C-byte transform at `+0x20` exists (building it
/// through callees 6 and 7 when null), and evaluates six window values
/// from the transform and the parameter block in the original's exact
/// float operation order, biased by plus or minus a global constant.
/// The six values (with a fill hole word between the third and fourth)
/// go to the window test (callee 8); a nonzero answer returns 1,
/// otherwise the scan continues with the next index and returns 0 when
/// no index passes.
///
/// Only the low byte of the return is meaningful (0 or 1). All float
/// arithmetic is single-precision in the original's operand order.
///
/// Original: 0x00DA9E60 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00DA9E60(arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const TAG_OFF: u32 = 0x2E;
        const TAG_EMPTY: u16 = 0xFFFF;
        const XF_OFF: u32 = 0x20;
        const TABLE: u32 = 0x1295CD8;
        const FLAG_OFF: u32 = 0x40;
        const FLAG_BIT: u32 = 0x10000;
        const TYPE_WANT: u8 = 0x0E;
        const BIAS: u32 = 0xFE8830;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let mut esi: u32 = lf_checker_rt::callee_cdecl!(0, u32, rd32(arg1));
        if esi == 0 || rd16(esi.wrapping_add(TAG_OFF)) == TAG_EMPTY {
            let mut buf = [0u32; 4];
            esi = lf_checker_rt::callee_cdecl!(1, u32, arg0, buf.as_mut_ptr() as u32, 0);
        }
        if esi == 0 || rd16(esi.wrapping_add(TAG_OFF)) == TAG_EMPTY {
            return 0;
        }
        let idx = rd16(esi.wrapping_add(TAG_OFF)) as u32;
        let edi = rd32(lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(4)));
        if edi == 0 {
            return 0;
        }
        if rd32(edi.wrapping_add(FLAG_OFF)) & FLAG_BIT == 0 {
            return 0;
        }
        let c0: u32 = lf_checker_rt::callee_thiscall!(2, u32, edi);
        if (c0 as i32) <= 0 {
            return 0;
        }
        let count: u32 = lf_checker_rt::callee_thiscall!(2, u32, edi);
        if (count as i32) <= 0 {
            return 0;
        }
        let g = f32::from_bits(rd32(lf_checker_rt::relocated(BIAS)));
        let mut i = 0u32;
        loop {
            let item: u32 = lf_checker_rt::callee_thiscall!(3, u32, edi, i);
            let vt = rd32(item);
            let type_q: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(4)) as usize);
            if (type_q(item) as u8) != TYPE_WANT {
                // Next index.
            } else {
                let mat_q: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(0x44)) as usize);
                let m = mat_q(item);
                let m10 = rdf(m.wrapping_add(0x10));
                let m14 = rdf(m.wrapping_add(0x14));
                let m18 = rdf(m.wrapping_add(0x18));
                let m1c = rdf(m.wrapping_add(0x1C));
                let m20 = rdf(m.wrapping_add(0x20));
                let m24 = rdf(m.wrapping_add(0x24));
                if rd32(esi.wrapping_add(XF_OFF)) == 0 {
                    let _a: u32 = lf_checker_rt::callee_thiscall!(6, u32, esi);
                    let _b: u32 = lf_checker_rt::callee_thiscall!(7, u32, esi.wrapping_add(0x10), 0);
                }
                let t = rd32(esi.wrapping_add(XF_OFF));
                let t0 = rdf(t);
                let t4 = rdf(t.wrapping_add(4));
                let t8 = rdf(t.wrapping_add(8));
                let t10 = rdf(t.wrapping_add(0x10));
                let t14 = rdf(t.wrapping_add(0x14));
                let t18 = rdf(t.wrapping_add(0x18));
                let t20 = rdf(t.wrapping_add(0x20));
                let t24 = rdf(t.wrapping_add(0x24));
                let t28 = rdf(t.wrapping_add(0x28));
                let t30 = rdf(t.wrapping_add(0x30));
                let t34 = rdf(t.wrapping_add(0x34));
                let t38 = rdf(t.wrapping_add(0x38));
                let x = add(add(add(mul(t0, m10), mul(t10, m14)), mul(t20, m18)), t30);
                let y = add(add(add(mul(t4, m10), mul(t14, m14)), mul(t24, m18)), t34);
                let z0 = add(add(add(mul(t8, m10), mul(t18, m14)), mul(t28, m18)), t38);
                let v5 = add(add(mul(t0, m1c), mul(t10, m20)), mul(t20, m24));
                let o3 = add(add(v5, t30), g);
                let w = add(add(add(add(mul(t4, m1c), mul(t14, m20)), mul(t24, m24)), t34), g);
                let o5 = add(add(add(add(mul(t8, m1c), mul(t18, m20)), mul(t28, m24)), t38), g);
                let mut out = [
                    sub(x, g).to_bits(), sub(y, g).to_bits(), sub(z0, g).to_bits(),
                    0,
                    o3.to_bits(), w.to_bits(), o5.to_bits(),
                ];
                let ok: u32 = lf_checker_rt::callee_thiscall!(8, u32,
                    out.as_mut_ptr() as u32, arg1.wrapping_add(0x10));
                if (ok as u8) != 0 {
                    return 1;
                }
            }
            i = i.wrapping_add(1);
            if (i as i32) >= (count as i32) {
                return 0;
            }
        }
    }
});
