// original: 0x00c2d390 MAIN_FIRE_LOOP

/// Fire-audio main loop step: silence, tail, or full re-voice by level.
///
/// `this` is the fire-sound object with voice slots at `+0x4C`, `+0x54`,
/// `+0x50`; `arg` is the requester (position source at `+0x20`, mix word
/// at `+0x64`); `level` is the current fire level. Three global gates
/// each abort silently: `ABORT_FLAG` set, `TICK_A` disagreeing with
/// `TICK_B`, or `MODE` holding `MODE_IDLE`.
///
/// Behaviour on the running path, by level against `FADE_END` (0.2):
/// - At or below the fade end (ordered comparison; NaN counts as above),
///   the quiet path: with no primary slot this returns at once, otherwise
///   each live slot's id word at `+0xA4` is released (callee 1) and the
///   three slots go to the silencer (callee 2, nine trailing zero words).
/// - Above the fade end with all three slots live, the tail path skips
///   straight to the shared ending: the volume updater (callee 15, the
///   updater itself) runs with `max(VOL_FLOOR, level)`, gain 1.0 and the
///   position vector.
/// - Above the fade end otherwise, the full path: each live slot's id is
///   released, the silencer runs, then a request block of 18 words is
///   initialized (callee 3) and customized (flag byte `FLAG_OFF` OR-ed
///   with `FLAG_BITS`, word 5 set to the copied position, word 9 to the
///   flavor answer of callee 4, word 11 to the rolling `COUNTER`).
///   Three voices open in turn (callees 5, 6, 7 with tags `TAG0..2`,
///   out-pointers to the three slots); each live voice is configured
///   with three zero words (callee 9), given two generated ids (callees
///   10 and 11), and stores ids-plus-zero at `+0xA4`/`+0xA8`/`+0xAC`
///   (the middle voice also takes a marker word through callee 8). A
///   closer call (callee 12, tag `TAG3`, no out-pointer) ends the
///   sequence. The counter advances by `COUNTER_STEP` modulo
///   `COUNTER_MOD` (signed remainder, stored back), nine constant voice
///   words go to the tuner (callee 13), and a final mix call (callee 14)
///   takes two reserved zero words, the third voice's id (or -1), the
///   mix word, and the flavor answer.
///
/// The two reserved mix words sit in frame slots the original never
/// writes; they read 0 under a zero stack fill. The request block's word
/// 5 holds a frame address, so snapshots skip it. Callee answers that
/// arrive in registers are scripted by the checker.
///
/// Original: 0x00c2d390 (thiscall, two stack words, no return value).
lf_checker_rt::export!(thiscall, rw_00c2d390(this: u32, arg: u32, level: u32) -> u32 {
    unsafe {
        const ABORT_FLAG: u32 = 0x11F7060;
        const TICK_A: u32 = 0x12088B4;
        const TICK_B: u32 = 0xF1C040;
        const MODE: u32 = 0x1037720;
        const MODE_IDLE: u32 = 0x12;
        const FADE_END: u32 = 0xFE87D0;
        const VOL_FLOOR: u32 = 0xFE87E4;
        const COUNTER: u32 = 0x16CFD48;
        const COUNTER_STEP: u32 = 0x46;
        const COUNTER_MOD: i32 = 100;
        const SLOT0: u32 = 0x4C;
        const SLOT1: u32 = 0x54;
        const SLOT2: u32 = 0x50;
        const POS_SRC: u32 = 0x20;
        const MIX_WORD: u32 = 0x64;
        const HANDLE_ID: u32 = 0xA4;
        const FLAG_OFF: usize = 0x46;
        const FLAG_BITS: u8 = 2;
        const TAG0: u32 = 0xEC6D74;
        const TAG1: u32 = 0xEC6D84;
        const TAG2: u32 = 0xEC6D94;
        const TAG3: u32 = 0xEC6DA4;
        const MARKER: u32 = 0xC2C80000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn release(slot: u32) {
            unsafe {
                if slot != 0 {
                    lf_checker_rt::callee_cdecl!(1, u32, rd32(slot.wrapping_add(HANDLE_ID)));
                }
            }
        }
        #[inline(always)]
        unsafe fn silence(this: u32, h0: u32, h1: u32, h2: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(2, u32, this, h0, h1, h2, 0, 0, 0, 0, 0, 0, 0, 0, 0);
            }
        }
        #[inline(always)]
        unsafe fn voice(h: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(9, u32, h, 0, 0, 0);
                let id0: u32 = lf_checker_rt::callee_thiscall!(10, u32, h);
                let id1: u32 = lf_checker_rt::callee_cdecl!(11, u32, id0);
                wr32(h.wrapping_add(0xA4), id0);
                wr32(h.wrapping_add(0xA8), id1);
                wr32(h.wrapping_add(0xAC), 0);
                id0
            }
        }

        if rd32(lf_checker_rt::relocated(ABORT_FLAG)) == 1 {
            return 0;
        }
        if rd32(lf_checker_rt::relocated(TICK_A)) != rd32(lf_checker_rt::relocated(TICK_B)) {
            return 0;
        }
        if rd32(lf_checker_rt::relocated(MODE)) == MODE_IDLE {
            return 0;
        }

        let s0 = rd32(this.wrapping_add(SLOT0));
        let s1 = rd32(this.wrapping_add(SLOT1));
        let s2 = rd32(this.wrapping_add(SLOT2));
        let fade = (lf_checker_rt::relocated(FADE_END) as *const f32).read_unaligned();
        let lvl = f32::from_bits(level);
        if fade >= lvl {
            if s0 == 0 {
                return 0;
            }
            release(s0);
            release(s1);
            release(s2);
            silence(this, s0, s1, s2);
            return 0;
        }
        if s0 == 0 || s1 == 0 || s2 == 0 {
            release(s0);
            release(s1);
            release(s2);
            silence(this, s0, s1, s2);

            let mut req = [0u32; 18];
            lf_checker_rt::callee_thiscall!(3, u32, req.as_mut_ptr() as u32);
            let psrc = rd32(arg.wrapping_add(POS_SRC));
            let mut pos = [0u32; 3];
            pos[0] = rd32(psrc.wrapping_add(0x30));
            pos[1] = rd32(psrc.wrapping_add(0x34));
            pos[2] = rd32(psrc.wrapping_add(0x38));
            *(req.as_mut_ptr() as *mut u8).add(FLAG_OFF) |= FLAG_BITS;
            req[5] = pos.as_mut_ptr() as u32;
            let flavor: u32 = lf_checker_rt::callee_thiscall!(4, u32, arg);
            req[9] = flavor;
            let seed = rd32(lf_checker_rt::relocated(COUNTER));
            req[11] = seed;

            let slot0 = this.wrapping_add(SLOT0);
            lf_checker_rt::callee_thiscall!(
                5, u32, this, lf_checker_rt::relocated(TAG0), slot0, req.as_mut_ptr() as u32,
                0xFFFF_FFFF, 0, 0
            );
            let h0 = rd32(slot0);
            if h0 != 0 {
                voice(h0);
            }
            let slot1 = this.wrapping_add(SLOT1);
            lf_checker_rt::callee_thiscall!(
                6, u32, this, lf_checker_rt::relocated(TAG1), slot1, req.as_mut_ptr() as u32,
                0xFFFF_FFFF, 0, 0
            );
            let h1 = rd32(slot1);
            if h1 != 0 {
                lf_checker_rt::callee_thiscall!(8, u32, h1, lf_checker_rt::relocated(MARKER));
                voice(h1);
            }
            let slot2 = this.wrapping_add(SLOT2);
            lf_checker_rt::callee_thiscall!(
                7, u32, this, lf_checker_rt::relocated(TAG2), slot2, req.as_mut_ptr() as u32,
                0xFFFF_FFFF, 0, 0
            );
            let h2 = rd32(slot2);
            let third = if h2 != 0 { voice(h2) } else { 0xFFFF_FFFF };

            lf_checker_rt::callee_thiscall!(
                12, u32, this, lf_checker_rt::relocated(TAG3), req.as_mut_ptr() as u32,
                0xFFFF_FFFF, 0, 0
            );

            let next = (seed.wrapping_add(COUNTER_STEP) as i32 % COUNTER_MOD) as u32;
            wr32(lf_checker_rt::relocated(COUNTER), next);
            lf_checker_rt::callee_thiscall!(
                13, u32, this.wrapping_add(0x0C),
                0x3DCCCCCD, 0x3DCCCCCD, 0, 0x3F000000, 0x3F4CCCCD, 0x3F800000, 0x3E99999A,
                0x3F000000, 0
            );
            let mixw = rd32(arg.wrapping_add(MIX_WORD));
            lf_checker_rt::callee_cdecl!(14, u32, 0, 0, third, mixw, flavor);
        }

        let floor = (lf_checker_rt::relocated(VOL_FLOOR) as *const f32).read_unaligned();
        let peak = if floor > lvl { floor } else { lvl };
        let base = rd32(arg.wrapping_add(POS_SRC));
        lf_checker_rt::callee_thiscall!(
            15, u32, this, peak.to_bits(), 0x3F800000, base.wrapping_add(0x30)
        );
        0
    }
});
