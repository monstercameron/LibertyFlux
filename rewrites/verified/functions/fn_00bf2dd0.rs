// original: 0x00BF2DD0 blend_matrix_rows
/// Blend one row set per index and report each through callee 5.
///
/// Resolves two working objects through the argument's function table
/// (slot 0xA0, then slot 0xE0 on the probe result, with an `obj+0x100`
/// fallback), then loops the index byte from 0x50 up to `this+0x17`.
/// Each iteration gathers one 0x40-stride source row selected by the
/// index, one matrix row selected by the row table's word entry, blends
/// them with scalar single-precision multiply-add chains in the
/// original's exact operand order, and hands the 14-word result buffer
/// plus the zero-based index to callee 5. Returns the last report
/// answer, or the row-table pointer when the loop runs zero times.
///
/// Cond: `this` points to a readable object with the count byte at
/// +0x17; `obj` to an object with a function table. The working objects
/// expose a float-matrix base at +0x14 and the row-table holder at +0x4.
export!(thiscall, rw_b157_f2(this: u32, obj: u32) -> u32 {
    unsafe {
        // Entry pair: register the object, then size it (answers unused).
        let _: u32 = callee_thiscall!(1, u32, this, obj, 1);
        let _: u32 = callee_thiscall!(2, u32, this, 0x1c);
        // Working pair: matrix-base object and row-table holder pointer.
                let res_a = {
            // Probe slot 0xA0; null falls back to obj+0x100, else fetch
            // again and dispatch slot 0xE0 (no stack args either way).
            let vt = *(obj as *const u32);
            let probe: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*(vt.wrapping_add(0xa0) as *const u32) as usize);
            if probe(obj) == 0 {
                *(obj.wrapping_add(0x100) as *const u32)
            } else {
                let fetch: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    *(vt.wrapping_add(0xa0) as *const u32) as usize,
                );
                let mid = fetch(obj);
                let vt2 = *(mid as *const u32);
                let tail: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    *(vt2.wrapping_add(0xe0) as *const u32) as usize,
                );
                tail(mid)
            }
        };
                let res_b = {
            // Probe slot 0xA0; null falls back to obj+0x100, else fetch
            // again and dispatch slot 0xE0 (no stack args either way).
            let vt = *(obj as *const u32);
            let probe: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*(vt.wrapping_add(0xa0) as *const u32) as usize);
            if probe(obj) == 0 {
                *(obj.wrapping_add(0x100) as *const u32)
            } else {
                let fetch: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    *(vt.wrapping_add(0xa0) as *const u32) as usize,
                );
                let mid = fetch(obj);
                let vt2 = *(mid as *const u32);
                let tail: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    *(vt2.wrapping_add(0xe0) as *const u32) as usize,
                );
                tail(mid)
            }
        };
        let ptr_p = *(res_b.wrapping_add(4) as *const u32);
        // Scratch frame. Slots the original never stores stay zero,
        // matching the checker's defined stack fill.
        let mut st = [0u32; 48];
        st[6] = res_a;
        st[7] = ptr_p;
        let count = *(this.wrapping_add(0x17) as *const u8);
        // Zero-iteration result; the low byte is fixed up after the loop.
        let mut ret = ptr_p;
        let mut idx: u8 = 0x50;
        // Scalar FP accumulators (the original's xmm0-xmm7 low lanes;
        // only scalar lanes are ever live) and integer temporaries.
        let (mut x0, mut x1, mut x2, mut x3): (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.0);
        let (mut x4, mut x5, mut x6, mut x7): (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.0);
        let (mut eax, mut ecx, mut edx, mut esi): (u32, u32, u32, u32) = (0, 0, 0, 0);
        while idx < count {
            ecx = idx as u32;
            eax = st[0x6];
            esi = ecx;
            ecx = ecx.wrapping_mul(0xe0);
            edx = *(eax.wrapping_add(0x14) as *const u32);
            eax = st[0x7];
            esi = esi.wrapping_shl(6);
            eax = *(eax as *const u32);
            x6 = *(esi.wrapping_add(edx) as *const f32);
            eax = *(ecx.wrapping_add(eax).wrapping_add(0x10) as *const u32);
            esi = esi.wrapping_add(edx);
            eax = *(eax.wrapping_add(0x14) as *const u16) as u32;
            st[0x20] = x6.to_bits();
            x4 = *(esi.wrapping_add(0x4) as *const f32);
            st[0x21] = x4.to_bits();
            x5 = *(esi.wrapping_add(0x8) as *const f32);
            st[0x22] = x5.to_bits();
            x0 = *(esi.wrapping_add(0x10) as *const f32);
            st[0x24] = x0.to_bits();
            x7 = *(esi.wrapping_add(0x14) as *const f32);
            st[0x8] = x0.to_bits();
            st[0x25] = x7.to_bits();
            x0 = *(esi.wrapping_add(0x18) as *const f32);
            st[0x26] = x0.to_bits();
            st[0xc] = x0.to_bits();
            x0 = *(esi.wrapping_add(0x20) as *const f32);
            st[0x28] = x0.to_bits();
            st[0x10] = x0.to_bits();
            x0 = *(esi.wrapping_add(0x24) as *const f32);
            st[0x29] = x0.to_bits();
            st[0xf] = x0.to_bits();
            x0 = *(esi.wrapping_add(0x28) as *const f32);
            st[0x2a] = x0.to_bits();
            st[0x16] = x0.to_bits();
            x0 = *(esi.wrapping_add(0x30) as *const f32);
            st[0x2c] = x0.to_bits();
            st[0x18] = x0.to_bits();
            x0 = *(esi.wrapping_add(0x34) as *const f32);
            st[0x2d] = x0.to_bits();
            st[0x1d] = x0.to_bits();
            x0 = *(esi.wrapping_add(0x38) as *const f32);
            st[0x2e] = x0.to_bits();
            st[0x1e] = x0.to_bits();
            eax = eax.wrapping_shl(6);
            eax = eax.wrapping_add(edx);
            x0 = x6;
            x0 = x0 * *(eax as *const f32);
            x2 = *(eax.wrapping_add(0x10) as *const f32);
            x1 = *(eax.wrapping_add(0x24) as *const f32);
            x3 = x4;
            x3 = x3 * *(eax.wrapping_add(0x4) as *const f32);
            x2 = x2 * x6;
            x3 = x3 + x0;
            x6 = x6 * *(eax.wrapping_add(0x20) as *const f32);
            x0 = x5;
            x0 = x0 * *(eax.wrapping_add(0x8) as *const f32);
            x1 = x1 * x4;
            x3 = x3 + x0;
            x0 = x4;
            x0 = x0 * *(eax.wrapping_add(0x14) as *const f32);
            x4 = f32::from_bits(st[0xc]);
            x1 = x1 + x6;
            x2 = x2 + x0;
            x0 = x5;
            x0 = x0 * *(eax.wrapping_add(0x18) as *const f32);
            x2 = x2 + x0;
            x0 = *(eax.wrapping_add(0x28) as *const f32);
            x0 = x0 * x5;
            x5 = f32::from_bits(st[0x8]);
            st[0x20] = x3.to_bits();
            x1 = x1 + x0;
            st[0x21] = x2.to_bits();
            x0 = x7;
            x3 = x5;
            st[0x22] = x1.to_bits();
            x0 = x0 * *(eax.wrapping_add(0x4) as *const f32);
            x3 = x3 * *(eax as *const f32);
            x3 = x3 + x0;
            x0 = x4;
            x0 = x0 * *(eax.wrapping_add(0x8) as *const f32);
            x3 = x3 + x0;
            x2 = *(eax.wrapping_add(0x10) as *const f32);
            x1 = *(eax.wrapping_add(0x24) as *const f32);
            x2 = x2 * x5;
            x5 = x5 * *(eax.wrapping_add(0x20) as *const f32);
            x0 = x7;
            x0 = x0 * *(eax.wrapping_add(0x14) as *const f32);
            x1 = x1 * x7;
            x2 = x2 + x0;
            x0 = x4;
            x0 = x0 * *(eax.wrapping_add(0x18) as *const f32);
            x1 = x1 + x5;
            x5 = f32::from_bits(st[0xf]);
            x2 = x2 + x0;
            x0 = *(eax.wrapping_add(0x28) as *const f32);
            x0 = x0 * x4;
            x4 = f32::from_bits(st[0x10]);
            st[0x25] = x2.to_bits();
            x1 = x1 + x0;
            st[0x24] = x3.to_bits();
            x3 = f32::from_bits(st[0x16]);
            x2 = x5;
            x0 = x4;
            st[0x26] = x1.to_bits();
            x0 = x0 * *(eax as *const f32);
            x2 = x2 * *(eax.wrapping_add(0x4) as *const f32);
            x1 = *(eax.wrapping_add(0x10) as *const f32);
            x1 = x1 * x4;
            x4 = x4 * *(eax.wrapping_add(0x20) as *const f32);
            x2 = x2 + x0;
            x0 = x3;
            x0 = x0 * *(eax.wrapping_add(0x8) as *const f32);
            x2 = x2 + x0;
            x0 = x5;
            x0 = x0 * *(eax.wrapping_add(0x14) as *const f32);
            x5 = x5 * *(eax.wrapping_add(0x24) as *const f32);
            x1 = x1 + x0;
            x0 = x3;
            x0 = x0 * *(eax.wrapping_add(0x18) as *const f32);
            x3 = x3 * *(eax.wrapping_add(0x28) as *const f32);
            x5 = x5 + x4;
            x4 = f32::from_bits(st[0x18]);
            x1 = x1 + x0;
            st[0x28] = x2.to_bits();
            x5 = x5 + x3;
            x3 = f32::from_bits(st[0x1e]);
            st[0x29] = x1.to_bits();
            st[0x2a] = x5.to_bits();
            x4 = x4 - *(eax.wrapping_add(0x30) as *const f32);
            x5 = f32::from_bits(st[0x1d]);
            st[0x2c] = x4.to_bits();
            x5 = x5 - *(eax.wrapping_add(0x34) as *const f32);
            st[0x2d] = x5.to_bits();
            x3 = x3 - *(eax.wrapping_add(0x38) as *const f32);
            x0 = x4;
            st[0x2e] = x3.to_bits();
            x0 = x0 * *(eax as *const f32);
            x2 = x5;
            x2 = x2 * *(eax.wrapping_add(0x4) as *const f32);
            x1 = x5;
            x1 = x1 * *(eax.wrapping_add(0x14) as *const f32);
            x5 = x5 * *(eax.wrapping_add(0x24) as *const f32);
            x2 = x2 + x0;
            x0 = x3;
            x0 = x0 * *(eax.wrapping_add(0x8) as *const f32);
            x2 = x2 + x0;
            x0 = x4;
            x0 = x0 * *(eax.wrapping_add(0x10) as *const f32);
            x4 = x4 * *(eax.wrapping_add(0x20) as *const f32);
            x1 = x1 + x0;
            x0 = x3;
            x0 = x0 * *(eax.wrapping_add(0x18) as *const f32);
            x3 = x3 * *(eax.wrapping_add(0x28) as *const f32);
            x5 = x5 + x4;
            x1 = x1 + x0;
            st[0x2c] = x2.to_bits();
            x5 = x5 + x3;
            st[0x2d] = x1.to_bits();
            let arg = idx.wrapping_sub(0x50) as u32;
            // The original pushes the index first and then takes
            // [esp+0x84], which is slot 0x80 of the loop body frame.
            ret = callee_thiscall!(5, u32, this, st.as_mut_ptr().add(32) as u32, arg);
            idx = idx.wrapping_add(1);
        }
        // The original reloads AL from the loop counter after the last
        // report, so the return value carries the final counter value in
        // its low byte (the start index 0x50 when the loop never runs).
        ret = (ret & 0xFFFFFF00) | idx as u32;
        ret
    }
});
