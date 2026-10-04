// original: 0x0088E090 aud_voice_fill_output
/// Fill this voice's output buffer from the queued per-lane chunks.
///
/// The voice keeps a ring of lanes; each lane has a source base, a progress
/// offset and a remaining count. This routine copies up to `total` units into
/// the output buffer (two chunk copies at most, one per lane visit), zero-pads
/// any shortfall, and advances the ring index past exhausted lanes.
///
/// `reset` selects the fill size: nonzero clears the sequence number and
/// fills 0x20000 units, zero keeps it, bumps a counter and fills 0x10000.
/// When the zero-pad path runs and the done flag is clear, a completion flag
/// is set. A zero `reset` also steps the sequence number 0 -> 1 -> 0.
///
/// Returns what the original leaves in EAX: the zero-pad call's answer
/// when padding runs, otherwise lane-index scratch or a division quotient
/// from the index step (the chunk-copy answers die in scratch), or the
/// stepped sequence number when `reset` is zero (only its low byte is read).
export!(thiscall, rw_88e090(this: *mut u8, reset: u32) -> u32 {
    unsafe {
        const SEQ: usize = 0x110;
        const FLAG: usize = 0x114;
        const INDEX: usize = 0x11C;
        const COUNT: usize = 0x120;
        const COUNTER: usize = 0x124;
        const OUT_BASE: usize = 0x138;
        const DONE: usize = 0x13C;
        const LANE_STRIDE: usize = 64;
        const LANE_SRC: usize = 0x90;
        const LANE_PROG: usize = 0xC8;
        const LANE_REMAIN: usize = 0xCC;

        #[inline]
        unsafe fn rd(base: *mut u8, off: usize) -> u32 {
            *(base.add(off) as *mut u32)
        }
        #[inline]
        unsafe fn wr(base: *mut u8, off: usize, v: u32) {
            *(base.add(off) as *mut u32) = v;
        }

        let total: u32 = if (reset & 0xFF) != 0 {
            wr(this, SEQ, 0);
            0x20000
        } else {
            wr(this, COUNTER, rd(this, COUNTER).wrapping_add(1));
            0x10000
        };
        // The original shifts the sequence left by 17 then logically right by
        // 1: bit 15 of the sequence shifts out of the register, so only the
        // low 15 bits survive, moved up by 16.
        let dst0: u32 = ((rd(this, SEQ) & 0x7FFF) << 16).wrapping_add(rd(this, OUT_BASE));
        // Always overwritten before return: with nonzero reset at least one
        // of the three calls below runs (the pad call runs whenever the two
        // chunk copies leave any shortfall); with zero reset the tail block
        // overwrites it. Mirrors the original, whose entry EAX is likewise
        // dead on every path.
        let mut eax: u32 = 0;

        let lane_off = |index: u32| (index as usize).wrapping_mul(LANE_STRIDE);
        // Step the ring index past an exhausted lane. The original divides,
        // so the quotient lands in EAX: it is observable on exit whenever no
        // later call or assignment overwrites it.
        let advance = |base: *mut u8, index: u32| -> u32 {
            let count = rd(base, COUNT);
            let next = index.wrapping_add(1);
            wr(base, INDEX, next % count);
            next / count
        };

        // First chunk: up to `total` units from the current lane.
        let mut n: u32 = total.min(rd(this, LANE_REMAIN + lane_off(rd(this, INDEX))));
        if n != 0 {
            let lane = lane_off(rd(this, INDEX));
            let src = rd(this, LANE_SRC + lane).wrapping_add(rd(this, LANE_PROG + lane));
            // The call answer dies immediately: the original reloads the lane
            // index into EAX as address scratch, so the surviving value is
            // the shifted index, or the division quotient below.
            let _answer: u32 = callee_cdecl!(1, u32, dst0, src, n);
            let idx = rd(this, INDEX);
            let lane = lane_off(idx);
            eax = idx << 6;
            wr(this, LANE_PROG + lane, rd(this, LANE_PROG + lane).wrapping_add(n));
            wr(this, LANE_REMAIN + lane, rd(this, LANE_REMAIN + lane).wrapping_sub(n));
            if rd(this, LANE_REMAIN + lane) == 0 {
                eax = advance(this, idx);
            }
        }
        // Second chunk: the rest of `total` from the (possibly next) lane.
        if n < total {
            eax = total.wrapping_sub(n);
            let lane = lane_off(rd(this, INDEX));
            let m: u32 = total.wrapping_sub(n).min(rd(this, LANE_REMAIN + lane));
            if m != 0 {
                let lane = lane_off(rd(this, INDEX));
                let src =
                    rd(this, LANE_SRC + lane).wrapping_add(rd(this, LANE_PROG + lane));
                let _answer: u32 =
                    callee_cdecl!(1, u32, dst0.wrapping_add(n), src, m);
                let idx = rd(this, INDEX);
                let lane = lane_off(idx);
                eax = idx << 6;
                wr(this, LANE_PROG + lane, rd(this, LANE_PROG + lane).wrapping_add(m));
                wr(this, LANE_REMAIN + lane, rd(this, LANE_REMAIN + lane).wrapping_sub(m));
                if rd(this, LANE_REMAIN + lane) == 0 {
                    eax = advance(this, idx);
                }
            }
            n = n.wrapping_add(m);
        }
        // Zero-pad any shortfall.
        if n < total {
            eax = callee_cdecl!(2, u32, dst0.wrapping_add(n), 0, total.wrapping_sub(n));
            if *this.add(DONE) == 0 {
                wr(this, FLAG, 1);
                *this.add(DONE) = 1;
            }
        }
        // Step the sequence number on the non-reset path.
        if (reset & 0xFF) == 0 {
            let stepped = rd(this, SEQ).wrapping_sub(1) & 1;
            wr(this, SEQ, stepped);
            eax = stepped;
        }
        eax
    }
});
