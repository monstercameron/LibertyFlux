// original: 0x006443A0 rage::ptxDomainVortex::vf5

/// Update a vortex particle domain for time `t`, sampling two keyframed
/// parameter tracks, notifying listeners, and rebuilding derived state.
///
/// `this` is the domain object. Track 1 (table pointer at `+0x28`, 16-bit
/// entry count at `+0x2c`) and track 2 (table at `+0x50`, count at `+0x54`)
/// hold 0x30-byte keyframes: time key at `+0x00`, base values at
/// `+0x10..+0x1c`, slopes at `+0x20..+0x2c`. Each track is sampled by
/// scanning from entry 1 for the first key not below `t` (an unordered
/// floating compare counts as below, so NaN keys are skipped) and
/// evaluating the previous entry as `slope * (t - key) + base` on four
/// channels; a track with fewer than two entries evaluates entry 0.
/// The sampled time is stored at `+0x114`.
///
/// When the listener count at `+0x11c` is non-zero, callee A (id 1) is
/// invoked twice with (index, result-frame pointer, pass 0 then 1, time,
/// object at `+0x100`), where index is the word at `+0x0c` plus one.
/// The first track's channels land at `+0xf0..+0xf8` (x, y, z twice: the
/// z slot is filled from a second spill of the same value) and the
/// second track's at `+0x130..+0x138`, then the first triple is scaled by
/// the factor at `+0x04`.
///
/// An identity 3x3 matrix plus a zero row is written at `+0xb0`, and
/// callee B (id 2) runs three times over it with the scaled second
/// triple times the shared constant: each call passes a frame pointer to
/// three words holding a unit axis ((1,0,0), then (1,0,1) and (1,0,0) as
/// the frame is only partly rewritten between calls) and one float.
/// The scaled triple is copied bit-wise to `+0xe0..+0xe8`.
///
/// When `ctx` is non-null and the flag byte at `+0x124` is clear, callee
/// E (id 3) runs over the matrix with `ctx`, and its answer is the
/// return value; otherwise the words at `ctx+0x30..+0x38` are added to
/// `+0xe0..+0xe8` when the flag is set, and subtracted again when the
/// flag byte at `+0x125` is set. A null `ctx` with the second flag set
/// faults on a null read, as does the original.
///
/// Original: 0x006443A0 (thiscall, two stack words: float time, context
/// pointer). Returns callee E's answer on the path that calls it, else
/// the bit pattern left in the z slot. All count compares are over
/// non-negative values, so signed and unsigned agree. Float operation
/// order is the original's.
lf_checker_rt::export!(thiscall, rw_006443a0(this: u32, t_bits: u32, ctx: u32) -> u32 {
    unsafe {
        const TRACK1_TABLE: u32 = 0x28;
        const TRACK1_COUNT: u32 = 0x2c;
        const TRACK2_TABLE: u32 = 0x50;
        const TRACK2_COUNT: u32 = 0x54;
        const ENTRY_STRIDE: u32 = 0x30;
        const KEY_OFF: u32 = 0x00;
        const BASE_OFF: u32 = 0x10;
        const SLOPE_OFF: u32 = 0x20;
        const SAMPLED_T: u32 = 0x114;
        const LISTENER_COUNT: u32 = 0x11c;
        const LISTENER_INDEX_SRC: u32 = 0x0c;
        const LISTENER_OBJ: u32 = 0x100;
        const SCALE: u32 = 0x04;
        const OUT_A: u32 = 0xf0;
        const OUT_B: u32 = 0x130;
        const MATRIX: u32 = 0xb0;
        const DERIVED: u32 = 0xe0;
        const FLAG_CALL: u32 = 0x124;
        const FLAG_SUB: u32 = 0x125;
        const CTX_OFF: u32 = 0x30;
        const SHARED_K: u32 = 0x00FE8728;
        const ONE_BITS: u32 = 0x3F800000;
        const CALLEE_A: u32 = 1;
        const CALLEE_B: u32 = 2;
        const CALLEE_E: u32 = 3;

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

        /// Sample one keyframe track at `t`: index of the entry to
        /// evaluate (first key not below `t`, minus one; entry 0 when
        /// the track has fewer than two entries).
        unsafe fn track_index(table: u32, count: u32, t: f32) -> u32 {
            unsafe {
                let mut idx = 1u32;
                if count > 1 {
                    let mut p = table.wrapping_add(ENTRY_STRIDE);
                    loop {
                        let key = rdf(p.wrapping_add(KEY_OFF));
                        // comiss + jae: unordered (NaN key) falls through
                        // exactly like "below", as does `>=` failing.
                        if key >= t {
                            break;
                        }
                        idx += 1;
                        p = p.wrapping_add(ENTRY_STRIDE);
                        if idx >= count {
                            break;
                        }
                    }
                }
                idx
            }
        }

        /// Evaluate entry `idx` of `table` at `t` into four channels.
        unsafe fn track_eval(table: u32, idx: u32, t: f32) -> [f32; 4] {
            unsafe {
                let entry = table
                    .wrapping_add(idx.wrapping_mul(3).wrapping_shl(4))
                    .wrapping_sub(ENTRY_STRIDE);
                let dt = sub(t, rdf(entry.wrapping_add(KEY_OFF)));
                [
                    add(mul(rdf(entry.wrapping_add(SLOPE_OFF)), dt), rdf(entry.wrapping_add(BASE_OFF))),
                    add(mul(rdf(entry.wrapping_add(SLOPE_OFF + 4)), dt), rdf(entry.wrapping_add(BASE_OFF + 4))),
                    add(mul(rdf(entry.wrapping_add(SLOPE_OFF + 8)), dt), rdf(entry.wrapping_add(BASE_OFF + 8))),
                    add(mul(rdf(entry.wrapping_add(SLOPE_OFF + 12)), dt), rdf(entry.wrapping_add(BASE_OFF + 12))),
                ]
            }
        }

        let t = f32::from_bits(t_bits);
        wrf(this.wrapping_add(SAMPLED_T), t);

        let count1 = rd16(this.wrapping_add(TRACK1_COUNT));
        let table1 = rd32(this.wrapping_add(TRACK1_TABLE));
        let a = track_eval(table1, track_index(table1, count1, t), t);

        let count2 = rd16(this.wrapping_add(TRACK2_COUNT));
        let table2 = rd32(this.wrapping_add(TRACK2_TABLE));
        let b = track_eval(table2, track_index(table2, count2, t), t);

        let n = rd32(this.wrapping_add(LISTENER_COUNT));
        if n != 0 {
            let index = rd32(this.wrapping_add(LISTENER_INDEX_SRC)).wrapping_add(1);
            let obj = this.wrapping_add(LISTENER_OBJ);
            let buf1 = [a[0].to_bits(), a[1].to_bits(), a[2].to_bits(), a[3].to_bits()];
            lf_checker_rt::callee_thiscall!(
                CALLEE_A, u32, n, index, (&buf1 as *const u32) as u32, 0, t_bits, obj
            );
            let buf2 = [b[0].to_bits(), b[1].to_bits(), b[2].to_bits(), b[3].to_bits()];
            lf_checker_rt::callee_thiscall!(
                CALLEE_A, u32, n, index, (&buf2 as *const u32) as u32, 1, t_bits, obj
            );
        }

        wrf(this.wrapping_add(OUT_A + 8), a[2]);
        wrf(this.wrapping_add(OUT_A), a[0]);
        wrf(this.wrapping_add(OUT_A + 4), a[1]);
        wrf(this.wrapping_add(OUT_B), b[0]);
        wrf(this.wrapping_add(OUT_B + 4), b[1]);
        wrf(this.wrapping_add(OUT_B + 8), b[2]);

        let sc = rdf(this.wrapping_add(SCALE));
        wrf(this.wrapping_add(OUT_A), mul(rdf(this.wrapping_add(OUT_A)), sc));
        wrf(this.wrapping_add(OUT_A + 4), mul(rdf(this.wrapping_add(OUT_A + 4)), sc));
        wrf(this.wrapping_add(OUT_A + 8), mul(rdf(this.wrapping_add(OUT_A + 8)), sc));

        let m = this.wrapping_add(MATRIX);
        wr32(m, ONE_BITS);
        wr32(m.wrapping_add(4), 0);
        wr32(m.wrapping_add(8), 0);
        wr32(m.wrapping_add(0x10), 0);
        wr32(m.wrapping_add(0x14), ONE_BITS);
        wr32(m.wrapping_add(0x18), 0);
        wr32(m.wrapping_add(0x20), 0);
        wr32(m.wrapping_add(0x24), 0);
        wr32(m.wrapping_add(0x28), ONE_BITS);
        wr32(m.wrapping_add(0x38), 0);
        wr32(m.wrapping_add(0x34), 0);
        wr32(m.wrapping_add(0x30), 0);

        let k = f32::from_bits(rd32(lf_checker_rt::relocated(SHARED_K)));
        let f1 = mul(rdf(this.wrapping_add(OUT_B)), k);
        let axis1 = [ONE_BITS, 0u32, 0u32];
        lf_checker_rt::callee_thiscall!(
            CALLEE_B, u32, m, (&axis1 as *const u32) as u32, f1.to_bits()
        );
        let f2 = mul(rdf(this.wrapping_add(OUT_B + 4)), k);
        let axis2 = [ONE_BITS, 0u32, ONE_BITS];
        lf_checker_rt::callee_thiscall!(
            CALLEE_B, u32, m, (&axis2 as *const u32) as u32, f2.to_bits()
        );
        let f3 = mul(rdf(this.wrapping_add(OUT_B + 8)), k);
        let axis3 = [ONE_BITS, 0u32, 0u32];
        lf_checker_rt::callee_thiscall!(
            CALLEE_B, u32, m, (&axis3 as *const u32) as u32, f3.to_bits()
        );

        let e0 = rd32(this.wrapping_add(OUT_A));
        let e1 = rd32(this.wrapping_add(OUT_A + 4));
        let e2 = rd32(this.wrapping_add(OUT_A + 8));
        wr32(this.wrapping_add(DERIVED), e0);
        wr32(this.wrapping_add(DERIVED + 4), e1);
        wr32(this.wrapping_add(DERIVED + 8), e2);

        let mut answer = e2;
        if ctx != 0 {
            if rd8(this.wrapping_add(FLAG_CALL)) == 0 {
                answer = lf_checker_rt::callee_thiscall!(CALLEE_E, u32, m, ctx);
            } else {
                wrf(
                    this.wrapping_add(DERIVED),
                    add(rdf(this.wrapping_add(DERIVED)), rdf(ctx.wrapping_add(CTX_OFF))),
                );
                wrf(
                    this.wrapping_add(DERIVED + 4),
                    add(rdf(this.wrapping_add(DERIVED + 4)), rdf(ctx.wrapping_add(CTX_OFF + 4))),
                );
                wrf(
                    this.wrapping_add(DERIVED + 8),
                    add(rdf(this.wrapping_add(DERIVED + 8)), rdf(ctx.wrapping_add(CTX_OFF + 8))),
                );
            }
        }
        if rd8(this.wrapping_add(FLAG_SUB)) != 0 {
            wrf(
                this.wrapping_add(DERIVED),
                sub(rdf(this.wrapping_add(DERIVED)), rdf(ctx.wrapping_add(CTX_OFF))),
            );
            wrf(
                this.wrapping_add(DERIVED + 4),
                sub(rdf(this.wrapping_add(DERIVED + 4)), rdf(ctx.wrapping_add(CTX_OFF + 4))),
            );
            wrf(
                this.wrapping_add(DERIVED + 8),
                sub(rdf(this.wrapping_add(DERIVED + 8)), rdf(ctx.wrapping_add(CTX_OFF + 8))),
            );
        }
        answer
    }
});
