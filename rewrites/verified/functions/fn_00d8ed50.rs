// original: 0x00D8ED50 audio_query_match_detail (proposed)

/// Build a spatial-audio query like `audio_query_emit`, then resolve the
/// match: either hand it to a detail call or interpolate a position from
/// table lookups.
///
/// Arguments mirror the simpler query (`this`, `pos` with four floats,
/// `center` radius, `out_row`, `tag`, `key_hi`, `key_lo`) plus a second
/// opaque table word `tag2` and a mode byte `mode`. The query setup is
/// identical: optional refresh (callee 1, flag
/// bit 2 at `this + 0x50`), struct initialisation (callee 2), six scaled
/// slot words `(pos[i] -/+ center) * 8`, twelve float slots, tail keys and a
/// `0xffff` marker. The table call is callee 3 when the word at `this +
/// 0x70` is zero, else callee 4; both write back the match index (and an
/// auxiliary word) which this function reads.
///
/// A `0xffff` index returns 0. Otherwise the match row
/// (`[this + 0x6c] + index * 40`) is derived and, when `mode` is nonzero,
/// passed with the output row to the detail call (callee 8) and returned.
/// When `mode` is zero, the row's flag word selects a divisor `d` (bits
/// 21-24, never zero in the contract) dividing `aux + 1`; quotient and
/// remainder index 16-bit table entries (through `[this + 0x60]`) for two
/// interpolation calls (callees 6 and 7, four floats each), whose results
/// are differenced, scaled by the factor call (callee 9, three pointers)
/// and added to the first call's outputs before being stored to the output
/// row. Callee 5 is the stack-cookie check. One word the original folds
/// into the query was never written by it (the checker's defined stack fill
/// on both sides); the second interpolation call later overwrites that same
/// slot with its fourth word, which the epilogue re-reads for the fourth
/// output.
///
/// Original: 0x00D8ED50 (thiscall, eight stack words).
lf_checker_rt::export!(thiscall, rw_00d8ed50(this: u32, pos: u32, center: u32, out_row: u32, tag2: u32, mode: u32, tag: u32, key_hi: u32, key_lo: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x50;
        const FLAG_REFRESH: u8 = 4;
        const ROW_BASE_OFF: u32 = 0x6c;
        const TABLE_SEL_OFF: u32 = 0x70;
        const SCALE_ADDR: u32 = 0x00fe8afc;
        const NO_MATCH: u32 = 0xffff;
        const ROW_STRIDE: u32 = 40;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let mut frame = [0u32; 0x180];
        let base = frame.as_mut_ptr() as u32;
        let slots = base + 0x20;
        let query = base + 0x60;

        if rd32(this + FLAG_OFF) as u8 & FLAG_REFRESH != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, this);
        }
        lf_checker_rt::callee_thiscall!(2, u32, query);

        let c = f32::from_bits(center);
        let k = f32::from_bits(rd32(lf_checker_rt::relocated(SCALE_ADDR)));
        let px = f32::from_bits(rd32(pos));
        let py = f32::from_bits(rd32(pos + 4));
        let pz = f32::from_bits(rd32(pos + 8));
        let pe = f32::from_bits(rd32(pos + 12));
        let undef = rd32(base + 0x5c);

        let dx = sub(px, c);
        let dy = sub(py, c);
        let dz = sub(pz, c);
        let sx = add(px, c);
        let sy = add(py, c);
        let sz = add(pz, c);
        wr32(query + 0x00, dx.to_bits());
        wr32(query + 0x04, dy.to_bits());
        wr32(query + 0x08, dz.to_bits());
        wr32(query + 0x0c, undef);
        wr32(query + 0x10, sx.to_bits());
        wr32(query + 0x14, sy.to_bits());
        wr32(query + 0x18, sz.to_bits());
        wr32(query + 0x1c, undef);
        wr32(query + 0x20, px.to_bits());
        wr32(query + 0x24, py.to_bits());
        wr32(query + 0x28, pz.to_bits());
        wr32(query + 0x2c, pe.to_bits());
        wr16(slots + 0x00, mul(dx, k) as i32 as u16);
        wr16(slots + 0x02, mul(sx, k) as i32 as u16);
        wr16(slots + 0x04, mul(dy, k) as i32 as u16);
        wr16(slots + 0x06, mul(sy, k) as i32 as u16);
        wr16(slots + 0x08, mul(dz, k) as i32 as u16);
        wr16(slots + 0x0a, mul(sz, k) as i32 as u16);
        // The low key sits just past the slot block and is copied into the
        // tail from there; the snapshot of the slot block observes it here.
        wr32(base + 0x2c, key_lo);

        wr32(base + 0x5b0, 0x7f7fffff);
        wr32(base + 0x5b4, mul(c, c).to_bits());
        wr32(base + 0x5b8, NO_MATCH);
        wr32(base + 0x5bc, 0);
        wr32(base + 0x5c0, key_hi);
        wr32(base + 0x5c4, rd32(base + 0x2c));

        if rd32(this + TABLE_SEL_OFF) == 0 {
            lf_checker_rt::callee_thiscall!(3, u32, this, slots, tag2, tag, query);
        } else {
            let sel = rd32(this + TABLE_SEL_OFF);
            lf_checker_rt::callee_thiscall!(4, u32, this, slots, sel, tag2, tag, query);
        }

        let index = rd32(base + 0x5b8);
        if index == NO_MATCH {
            lf_checker_rt::callee_cdecl!(5, u32,);
            return 0;
        }
        let row = rd32(this + ROW_BASE_OFF).wrapping_add(index.wrapping_mul(ROW_STRIDE));
        wr32(base + 0x10, row);
        if (mode as u8) != 0 {
            lf_checker_rt::callee_thiscall!(8, u32, this, row, out_row);
            lf_checker_rt::callee_cdecl!(5, u32,);
            return row;
        }

        // Mode-zero epilogue: divisor from the row flags, table lookups,
        // two interpolation calls, differenced and scaled outputs.
        let aux = rd32(base + 0x5bc);
        let divisor = (rd32(row) >> 0x15) & 0xf;
        let _quot = (aux.wrapping_add(1)) / divisor;
        let rem = (aux.wrapping_add(1)) % divisor;
        let table = rd32(this + 0x60);
        let key1 = (rd32(row + 4) & 0x1ffff).wrapping_add(aux);
        let entry1 = rd16(table.wrapping_add(key1.wrapping_mul(2))) as u32;
        let out1 = base + 0x30;
        lf_checker_rt::callee_thiscall!(6, u32, this, entry1, out1);
        let key2 = (rd32(out_row + 4) & 0x1ffff).wrapping_add(rem);
        let entry2 = rd16(table.wrapping_add(key2.wrapping_mul(2))) as u32;
        let out2 = base + 0x50;
        lf_checker_rt::callee_thiscall!(7, u32, this, entry2, out2);

        let f30 = f32::from_bits(rd32(out1));
        let f34 = f32::from_bits(rd32(out1 + 4));
        let f38 = f32::from_bits(rd32(out1 + 8));
        let g50 = f32::from_bits(rd32(out2));
        let g54 = f32::from_bits(rd32(out2 + 4));
        let g58 = f32::from_bits(rd32(out2 + 8));
        let d0 = sub(g50, f30);
        let d1 = sub(g54, f34);
        let d2 = sub(g58, f38);
        wr32(base + 0x40, d0.to_bits());
        wr32(base + 0x44, d1.to_bits());
        wr32(base + 0x48, d2.to_bits());
        let t: f32 = lf_checker_rt::callee_cdecl!(9, f32, out1, base + 0x40, pos);
        wr32(out_row, add(mul(d0, t), f30).to_bits());
        wr32(out_row + 4, add(mul(d1, t), f34).to_bits());
        wr32(out_row + 8, add(mul(d2, t), f38).to_bits());
        // The fourth output re-reads the slot the second interpolation call
        // just overwrote with its fourth word (the earlier unfilled value
        // survives only in the two query slots written before the calls).
        wr32(out_row + 12, rd32(base + 0x5c));
        lf_checker_rt::callee_cdecl!(5, u32,);
        row
    }
});
