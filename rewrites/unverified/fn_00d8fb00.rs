// original: 0x00D8FB00 audio_query_emit (proposed)

/// Build a spatial-audio query from a listener position and run it against
/// the voice table, emitting the best match's position.
///
/// `this` is the audio object; `center` is a radius; `pos` points at four
/// floats (x, y, z, extra); `out` receives four floats on success. `tag`,
/// `key_hi` and `key_lo` are opaque words threaded into the query. Returns 0
/// when the table reports no match, otherwise a pointer derived from the
/// match index (`base + index * 40` with `base` at `this + 0x6c`).
///
/// Behaviour: unless bit 2 of the flag byte at `this + 0x50` is clear, a
/// refresh call runs first (callee 1). Callee 2 then initialises a query
/// struct on the stack. The rewrite fills the query's six 16-bit slots with
/// `(pos[i] -/+ center) * 8`, truncated toward zero: differences at words
/// 0x10/0x14/0x18 (low halves) and sums at 0x12/0x16/0x1a... laid out as
/// pairs (diff, sum) per axis. Twelve float slots hold the raw differences,
/// sums and position copies, and the tail holds `center * center`, the keys
/// and a `0xffff` "no match yet" marker. Which table call runs depends on
/// the word at `this + 0x70`: zero selects callee 3, nonzero callee 4; both
/// take the slot block and the query struct plus the opaque words, and write
/// back four floats and the match index. A `0xffff` index returns 0,
/// otherwise the four floats are copied to `out` and the derived pointer is
/// returned. Callee 5 is the stack-cookie check, called after the result is
/// set. Two words the original reads were never written by it (one folded
/// into the query, one passed through to the table call); both sides read
/// them as the checker's defined stack fill.
///
/// Original: 0x00D8FB00 (thiscall, seven stack words; the fourth is written
/// to the frame and overwritten before any read).
lf_checker_rt::export!(thiscall, rw_00d8fb00(this: u32, pos: u32, center: u32, out: u32, _a4: u32, tag: u32, key_hi: u32, key_lo: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x50;
        const FLAG_REFRESH: u8 = 4;
        const RET_BASE_OFF: u32 = 0x6c;
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

        // Query frame, mirroring the original's stack layout: slot block at
        // +0x10, query struct at +0x30, tail at +0x580. Zeroed like the
        // checker's defined stack fill, so the two words the original reads
        // without writing are observed identically on both sides.
        let mut frame = [0u32; 0x170];
        let base = frame.as_mut_ptr() as u32;
        let slots = base + 0x10;
        let query = base + 0x30;

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
        let undef = rd32(base + 0x1c);

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

        wr32(base + 0x580, 0x7f7fffff);
        wr32(base + 0x584, mul(c, c).to_bits());
        wr32(base + 0x588, NO_MATCH);
        wr32(base + 0x58c, 0);
        wr32(base + 0x590, key_hi);
        wr32(base + 0x594, key_lo);

        let through = rd32(base + 0x0c);
        if rd32(this + TABLE_SEL_OFF) == 0 {
            lf_checker_rt::callee_thiscall!(3, u32, this, slots, through, tag, query);
        } else {
            let sel = rd32(this + TABLE_SEL_OFF);
            lf_checker_rt::callee_thiscall!(4, u32, this, slots, sel, through, tag, query);
        }

        let index = rd32(base + 0x588);
        let result = if index == NO_MATCH {
            0u32
        } else {
            wr32(out, rd32(base + 0x60));
            wr32(out + 4, rd32(base + 0x64));
            wr32(out + 8, rd32(base + 0x68));
            wr32(out + 12, rd32(base + 0x6c));
            rd32(this + RET_BASE_OFF).wrapping_add(index.wrapping_mul(ROW_STRIDE))
        };
        lf_checker_rt::callee_cdecl!(5, u32,);
        result
    }
});
