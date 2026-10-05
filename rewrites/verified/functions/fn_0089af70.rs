// original: 0x0089AF70 aud_envelope_stage_update (proposed)

/// Advance the envelope stage machine of an audio sound.
///
/// `this` is an audio sound object and the single stack word is a signed
/// position `pos`. A probe helper (callee 0, `thiscall` on `this` with
/// five out-pointers) fills three live values: a status word, a cursor
/// and a magnitude (the other two out-words are never read). The status
/// word, the stage byte at `+0xF6` and the signed comparison of `pos`
/// against the limit at `+0xC8` select a fast path (negative status,
/// stage 4, or `pos` at/below the limit skip it): the fast path seeds
/// `+0xCC`/`+0xD0` from the limit plus the cursor, forces stage 4, sets
/// the levels at `+0xB0`/`+0xB8` to 1 and the level at `+0xB4` to the
/// magnitude widened to float times the global tick. The dispatch on the
/// stage byte then runs: stage 4 with a negative cursor returns 1; with
/// `pos` past the `+0xD0` mark it flags `+0xF8` and returns 0, otherwise
/// the blend helper (callee 1, `thiscall` on the mix row at `+0x50` with
/// `0, 1, (float)+0xCC, (float)+0xD0, (float)pos`) answers the level at
/// `+0xB8` and 1 returns. Stage 5 writes constant levels and returns 1.
/// Any other stage compares `pos` against `+0xC4` (signed): at or past it
/// the magnitude path runs (stage 3, `+0xB0` to 1, `+0xB4` to widened
/// magnitude times tick); below it `+0xBC` is compared (signed): below
/// that the levels clear (stage 0) while at or past it the `+0xC0` mark
/// selects blend call two (`0, 1, (float)+0xBC, (float)+0xC0,
/// (float)pos` on the mix row, stage 1, answer to `+0xB0` with `+0xB4`
/// forced to 1) or blend call three (widened magnitude times tick, `1`,
/// `(float)+0xC0`, `(float)+0xC4`, `(float)pos` on the mix row at
/// `+0x28`, stage 2, answer to `+0xB4` with `+0xB0` forced to 1). Every
/// tail path sets `+0xB8` to 1 and returns 1. All five position
/// comparisons are SIGNED (`jl`/`jle`/`jge`); integer-to-float widening
/// is signed except the magnitude, which is unsigned. The mix row is
/// `stride * byte(+0xF7) + row_base[byte(+0x40)]` over the global table.
/// The probe's out-pointers aim at the caller's own frame; their
/// addresses are skipped and the scripted out-words observed through the
/// cursor, status and magnitude uses instead.
///
/// Original: 0x0089AF70 (thiscall, one stack word, returns `al`).
lf_checker_rt::export!(thiscall, rw_0089AF70(this: u32, pos: u32) -> u32 {
    unsafe {
        const CAT_INDEX: u32 = 0x40;
        const STAGE: u32 = 0xF6;
        const ROW_BYTE: u32 = 0xF7;
        const FLAG_BYTE: u32 = 0xF8;
        const LIMIT: u32 = 0xC8;
        const SEED: u32 = 0xCC;
        const MARK: u32 = 0xD0;
        const MARK_B: u32 = 0xC4;
        const MARK_C: u32 = 0xBC;
        const MARK_D: u32 = 0xC0;
        const LEVEL_A: u32 = 0xB0;
        const LEVEL_B: u32 = 0xB4;
        const LEVEL_C: u32 = 0xB8;
        const CAT_STRIDE: u32 = 0x6F40;
        const TABLE_ROW: u32 = 0x6F14;
        const ONE_BITS: u32 = 0x3F800000;
        const G_TICK: u32 = 0xFE870C;
        const G_STRIDE: u32 = 0x115D968;
        const G_TABLE_BASE: u32 = 0x115D988;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        // Probe: five out-words, of which the last three are live.
        let mut o0 = 0u32;
        let mut o1 = 0u32;
        let mut o2 = 0u32;
        let mut o3 = 0u32;
        let mut o4 = 0u32;
        lf_checker_rt::callee_thiscall!(
            0,
            u32,
            this,
            &mut o0 as *mut u32 as u32,
            &mut o1 as *mut u32 as u32,
            &mut o2 as *mut u32 as u32,
            &mut o3 as *mut u32 as u32,
            &mut o4 as *mut u32 as u32
        );
        let _ = (o0, o1);
        let magnitude = o2;
        let status = o3 as i32;
        let cursor = o4;
        let tick = f32::from_bits(rd32(lf_checker_rt::relocated(G_TICK)));
        // Unsigned magnitude widened to float, times the tick.
        let scaled = mul(magnitude as f32, tick);
        let stride = (lf_checker_rt::global::<u32>(G_STRIDE)).read_unaligned();
        let table_base = (lf_checker_rt::global::<u32>(G_TABLE_BASE)).read_unaligned();
        let cat = rd8(this + CAT_INDEX) as u32;
        let mix_row = stride
            .wrapping_mul(rd8(this + ROW_BYTE) as u32)
            .wrapping_add(rd32(
                table_base.wrapping_add(cat.wrapping_mul(CAT_STRIDE)).wrapping_add(TABLE_ROW),
            ));
        let spos = pos as i32;

        // Fast path unless: negative status, stage 4, or pos at/below limit.
        if !(status < 0 || rd8(this + STAGE) == 4 || spos <= rd32(this + LIMIT) as i32) {
            let limit = rd32(this + LIMIT);
            wr32(this + SEED, limit);
            wr32(this + MARK, limit.wrapping_add(cursor));
            ((this + STAGE) as *mut u8).write(4);
            wr32(this + LEVEL_A, ONE_BITS);
            wr32(this + LEVEL_C, ONE_BITS);
            wr32(this + LEVEL_B, scaled.to_bits());
        }
        let stage = rd8(this + STAGE);
        if stage == 4 {
            if (cursor as i32) < 0 {
                ((this + STAGE) as *mut u8).write(4);
                return 1;
            }
            if spos <= rd32(this + MARK) as i32 {
                let level: f32 = lf_checker_rt::callee_thiscall!(
                    1,
                    f32,
                    mix_row.wrapping_add(0x50),
                    0,
                    ONE_BITS,
                    ((rd32(this + SEED) as i32) as f32).to_bits(),
                    ((rd32(this + MARK) as i32) as f32).to_bits(),
                    (spos as f32).to_bits()
                );
                wr32(this + LEVEL_C, level.to_bits());
                ((this + STAGE) as *mut u8).write(4);
                return 1;
            }
            let fb = (this + FLAG_BYTE) as *mut u8;
            fb.write(fb.read() | 4);
            return 0;
        }
        if stage == 5 {
            wr32(this + LEVEL_A, ONE_BITS);
            wr32(this + LEVEL_B, ONE_BITS);
            wr32(this + LEVEL_C, 0);
            return 1;
        }
        if spos >= rd32(this + MARK_B) as i32 {
            ((this + STAGE) as *mut u8).write(3);
            wr32(this + LEVEL_A, ONE_BITS);
            wr32(this + LEVEL_B, scaled.to_bits());
        } else if spos < rd32(this + MARK_C) as i32 {
            ((this + STAGE) as *mut u8).write(0);
            wr32(this + LEVEL_A, 0);
            wr32(this + LEVEL_B, ONE_BITS);
        } else if spos < rd32(this + MARK_D) as i32 {
            ((this + STAGE) as *mut u8).write(1);
            let level: f32 = lf_checker_rt::callee_thiscall!(
                1,
                f32,
                mix_row,
                0,
                ONE_BITS,
                ((rd32(this + MARK_C) as i32) as f32).to_bits(),
                ((rd32(this + MARK_D) as i32) as f32).to_bits(),
                (spos as f32).to_bits()
            );
            wr32(this + LEVEL_B, ONE_BITS);
            wr32(this + LEVEL_A, level.to_bits());
        } else {
            ((this + STAGE) as *mut u8).write(2);
            wr32(this + LEVEL_A, ONE_BITS);
            let level: f32 = lf_checker_rt::callee_thiscall!(
                1,
                f32,
                mix_row.wrapping_add(0x28),
                scaled.to_bits(),
                ONE_BITS,
                ((rd32(this + MARK_D) as i32) as f32).to_bits(),
                ((rd32(this + MARK_B) as i32) as f32).to_bits(),
                (spos as f32).to_bits()
            );
            wr32(this + LEVEL_B, level.to_bits());
        }
        wr32(this + LEVEL_C, ONE_BITS);
        1
    }
});
