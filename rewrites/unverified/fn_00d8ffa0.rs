// original: 0x00D8FFA0 audio_voice_pick (proposed)

/// Pick the best voice row for a query: filter rows by bounds, score the
/// survivors by distance, and keep the nearest.
///
/// `slots` holds six 16-bit bound pairs, `table` selects the row set,
/// `callback` (nullable) pre-filters each candidate row, `flags` bit 0x80
/// doubles the depth axis weight, and `buf` is the working buffer that
/// receives the winning row's point and index. Returns nothing.
///
/// When the table's set word (+0x2c) is null, the four quad entries
/// (+0x30..) are tested against the buffer's float box instead, and each
/// entry inside the box recurses (callee 5) with the remaining-entry count.
/// Otherwise each index of the set's array is resolved to a row
/// (`[this + 0x6c] + index * 40`) and kept only if its six bound words
/// satisfy the signed comparisons against the slots. A kept row runs the
/// callback (skipped on a zero answer), up to ((flag word >> 0x15) & 0xf)
/// interpolator calls (callee 1, skipped when the flag word has none of
/// bits 0x1e00000), and the point-fill call (callee 2, six words), whose
/// answer counts distance-loop iterations. Each iteration scores one point
/// against the buffer centre (squared distance, depth doubled under the
/// flag), and a score below both the threshold (+0x554) and the best so far
/// (+0x550) is stored with the point and row index, subject to the gate word
/// (+0x560) and the confirm call (callee 3). Callee 4 is the row callback.
///
/// Original: 0x00D8FFA0 (thiscall, five stack words, no return value; one
/// register-indirect callee, four direct including itself).
lf_checker_rt::export!(thiscall, rw_00d8ffa0(this: u32, slots: u32, table: u32, callback: u32, flags: u32, buf: u32) -> () {
    unsafe {
        const ROW_STRIDE: u32 = 40;
        const F_MASK: u32 = 0x1e00000;
        const DEPTH_BIT: u32 = 0x80;
        const DEPTH_SCALE_ADDR: u32 = 0x00fe8a24;

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
        #[inline(always)]
        unsafe fn keep_row(row: u32, slots: u32) -> bool {
            unsafe {
                (rd16(row + 0x10) as i16) <= (rd16(slots + 2) as i16)
                    && (rd16(row + 0x14) as i16) <= (rd16(slots + 6) as i16)
                    && (rd16(row + 0x18) as i16) <= (rd16(slots + 0x0a) as i16)
                    && (rd16(row + 0x12) as i16) >= (rd16(slots + 0) as i16)
                    && (rd16(row + 0x16) as i16) >= (rd16(slots + 4) as i16)
                    && (rd16(row + 0x1a) as i16) >= (rd16(slots + 8) as i16)
            }
        }

        let set = rd32(table + 0x2c);
        if set == 0 {
            // Quad path: box-test the four entries, recurse on each hit.
            let bx0 = f32::from_bits(rd32(buf));
            let bx4 = f32::from_bits(rd32(buf + 4));
            let bx10 = f32::from_bits(rd32(buf + 0x10));
            let bx14 = f32::from_bits(rd32(buf + 0x14));
            let mut left = 4u32;
            let mut p = table + 0x30;
            loop {
                let qp = rd32(p);
                if qp != 0 {
                    let inside = !(bx0
                        > f32::from_bits(rd32(qp + 0x10)))
                        && !(bx4 > f32::from_bits(rd32(qp + 0x14)))
                        && !(f32::from_bits(rd32(qp)) > bx10)
                        && !(f32::from_bits(rd32(qp + 4)) > bx14);
                    if inside {
                        lf_checker_rt::callee_thiscall!(5, u32, this, slots, qp, callback, flags, buf);
                    }
                }
                p = p.wrapping_add(4);
                left = left.wrapping_sub(1);
                if left == 0 {
                    break;
                }
            }
            return;
        }
        let count = rd16(set + 0xc) as u32;
        if (count as i32) <= 0 {
            return;
        }
        let arr = rd32(set + 4);
        let dbl = f32::from_bits(rd32(lf_checker_rt::relocated(DEPTH_SCALE_ADDR)));
        let row_base = rd32(this + 0x6c);
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            let idx = rd16(arr + i.wrapping_mul(2)) as u32;
            let row = row_base.wrapping_add(idx.wrapping_mul(ROW_STRIDE));
            i = i.wrapping_add(1);
            if !keep_row(row, slots) {
                continue;
            }
            if callback != 0 {
                let f: extern "cdecl" fn(u32, u32) -> u32 =
                    core::mem::transmute(callback as usize);
                if (f(this, row) as u8) == 0 {
                    continue;
                }
            }
            let fw = rd32(row);
            if (fw & F_MASK) != 0 {
                let iters = (fw >> 0x15) & 0xf;
                let base_k = rd32(row + 4) & 0x1ffff;
                let tbl16 = rd32(this + 0x60);
                let mut j = 0u32;
                let mut out = buf.wrapping_add(0x40);
                while j < iters {
                    let e = rd16(tbl16 + base_k.wrapping_add(j).wrapping_mul(2)) as u32;
                    lf_checker_rt::callee_thiscall!(1, u32, this, e, out);
                    out = out.wrapping_add(0x10);
                    j = j.wrapping_add(1);
                }
            }
            let k2 = rd32(row + 4) & 0x1ffff;
            let p3 = rd32(this + 0x64).wrapping_add(k2.wrapping_mul(8));
            // Fifth word is the flag byte the row header set earlier (bit 1
            // of the flag word); its upper bytes were never written, so both
            // sides read the checker's defined stack fill there.
            let flag = (rd32(row) >> 1) & 1;
            let n = lf_checker_rt::callee_cdecl!(2, u32, row, buf.wrapping_add(0x40), p3, 0, flag, buf.wrapping_add(0x140));
            let cx = f32::from_bits(rd32(buf + 0x20));
            let cy = f32::from_bits(rd32(buf + 0x24));
            let cz = f32::from_bits(rd32(buf + 0x28));
            let mut c = n;
            let mut fp = buf.wrapping_add(0x148);
            while c != 0 {
                let px = f32::from_bits(rd32(fp.wrapping_sub(8)));
                let py = f32::from_bits(rd32(fp.wrapping_sub(4)));
                let pz = f32::from_bits(rd32(fp));
                let dx = sub(cx, px);
                let dy = sub(cy, py);
                let mut dz = sub(cz, pz);
                if (flags & DEPTH_BIT) != 0 {
                    dz = mul(dz, dbl);
                }
                let d2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
                let thresh = f32::from_bits(rd32(buf + 0x554));
                if d2 < thresh {
                    let best = f32::from_bits(rd32(buf + 0x550));
                    if d2 < best {
                        let gate = rd32(buf + 0x560);
                        let ok = if gate == 0 {
                            true
                        } else {
                            let r: u32 = lf_checker_rt::callee_cdecl!(3, u32, fp.wrapping_sub(8), buf);
                            (r as u8) != 0
                        };
                        if ok {
                            wr32(buf + 0x550, d2.to_bits());
                            wr32(buf + 0x30, px.to_bits());
                            wr32(buf + 0x34, py.to_bits());
                            wr32(buf + 0x38, pz.to_bits());
                            wr32(buf + 0x3c, rd32(fp.wrapping_add(4)));
                            wr32(buf + 0x558, idx);
                        }
                    }
                }
                fp = fp.wrapping_add(0x10);
                c = c.wrapping_sub(1);
            }
        }
    }
});
