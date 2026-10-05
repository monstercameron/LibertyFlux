// original: 0x00c2d090 fire_sound_update (proposed)

/// Per-tick update of a fire sound's three emitter slots, then one mix call.
///
/// `this` points to the fire-sound object with emitter slots at `+0x4C`,
/// `+0x54` and `+0x50` (each null when its voice is silent). `level` is a
/// loudness input, `gain` a gain input, and `pos` points at three position
/// words that are copied to the frame and handed to the final mix call.
///
/// Behaviour, in order:
/// - Three global gates each abort the update silently: `ABORT_FLAG` set,
///   `TICK_A` disagreeing with `TICK_B`, or `MODE` holding `MODE_IDLE`.
/// - `level` runs through a helper (callee 1) and a float filter (callee 2,
///   first call), giving `base`. A second helper (callee 3, constant mode 8)
///   gives `spread`; the gain used downstream is
///   `max(GAIN_FLOOR, gain) * spread` filtered again (callee 2, second
///   call), giving `shaped`. The maximum keeps the floor only on an
///   ordered greater-than, matching the original's `comiss`/`ja`.
/// - Each live slot is announced (callee 4), then resolves a voice id: byte
///   `VOICE` at `+0x04`, or voice 0 when it is `0xFF`; otherwise
///   `SCALE * voice + TABLE[band]` where `SCALE` is global, `band` is byte
///   `+0x40` selecting a `BAND_STRIDE` entry past global `TABLE_BASE`, and
///   the table word sits at `TABLE_DATA_OFF` into the entry. The slot's
///   float (`base` for slots 0 and 2, `shaped + base` in that order for
///   slot 1) and voice go to the voice setter (callee 5), and the slot's
///   handle at `+0xA4` is kept.
/// - The mix call (callee 6) takes the three handles (missing slot 0 keeps
///   its `-1` sentinel, a missing slot 2 passes `-1` explicitly, a missing
///   slot 1 passes whatever its uninitialized frame slot holds, which is 0
///   under a zero stack fill), then `base`, `shaped`, and the copied
///   position words.
///
/// Float helper answers arrive on the x87 stack; the checker scripts them.
/// This rewrite pins the two arithmetic orders of the original
/// (`floor-or-gain times spread`, `shaped plus base`).
///
/// Original: 0x00c2d090 (thiscall, three stack words, no return value).
lf_checker_rt::export!(thiscall, rw_00c2d090(this: u32, level: u32, gain: u32, pos: u32) -> u32 {
    unsafe {
        const ABORT_FLAG: u32 = 0x11F7060;
        const TICK_A: u32 = 0x12088B4;
        const TICK_B: u32 = 0xF1C040;
        const MODE: u32 = 0x1037720;
        const MODE_IDLE: u32 = 0x12;
        const GAIN_FLOOR: u32 = 0xFE8830;
        const SCALE: u32 = 0x115D968;
        const TABLE_BASE: u32 = 0x115D988;
        const MODE_CONST: u32 = 0x1289230;
        const MODE_ARG: u32 = 8;
        const SLOT0: u32 = 0x4C;
        const SLOT1: u32 = 0x54;
        const SLOT2: u32 = 0x50;
        const VOICE: u32 = 0x04;
        const BAND: u32 = 0x40;
        const HANDLE: u32 = 0xA4;
        const NO_VOICE: u32 = 0xFF;
        const BAND_STRIDE: u32 = 0x6F40;
        const TABLE_DATA_OFF: u32 = 0x6F14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read_unaligned() }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn voice_of(slot: u32) -> u32 {
            unsafe {
                let v = rd8(slot.wrapping_add(VOICE)) as u32;
                if v == NO_VOICE {
                    return 0;
                }
                let band = rd8(slot.wrapping_add(BAND)) as u32;
                let scale = rd32(lf_checker_rt::relocated(SCALE));
                let base = rd32(lf_checker_rt::relocated(TABLE_BASE));
                let entry = base
                    .wrapping_add(band.wrapping_mul(BAND_STRIDE))
                    .wrapping_add(TABLE_DATA_OFF);
                scale.wrapping_mul(v).wrapping_add(rd32(entry))
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

        let shaped_in: u32 = lf_checker_rt::callee_cdecl!(1, u32, level);
        let base: u32 = lf_checker_rt::callee_cdecl!(2, u32, shaped_in);
        let p0 = rd32(pos);
        let p1 = rd32(pos.wrapping_add(4));
        let p2 = rd32(pos.wrapping_add(8));
        let spread: u32 =
            lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(MODE_CONST), MODE_ARG);
        let floor = (lf_checker_rt::relocated(GAIN_FLOOR) as *const f32).read_unaligned();
        let gain_f = f32::from_bits(gain);
        let peak = if floor > gain_f { floor } else { gain_f };
        let shaped: u32 =
            lf_checker_rt::callee_cdecl!(2, u32, fmul(peak, f32::from_bits(spread)).to_bits());

        let s0 = rd32(this.wrapping_add(SLOT0));
        let h0 = if s0 != 0 {
            lf_checker_rt::callee_thiscall!(4, u32, s0, pos);
            lf_checker_rt::callee_thiscall!(5, u32, voice_of(s0), base);
            rd32(s0.wrapping_add(HANDLE))
        } else {
            0xFFFF_FFFF
        };
        let s1 = rd32(this.wrapping_add(SLOT1));
        let h1 = if s1 != 0 {
            lf_checker_rt::callee_thiscall!(4, u32, s1, pos);
            let mix = fadd(f32::from_bits(shaped), f32::from_bits(base)).to_bits();
            lf_checker_rt::callee_thiscall!(5, u32, voice_of(s1), mix);
            rd32(s1.wrapping_add(HANDLE))
        } else {
            0
        };
        let s2 = rd32(this.wrapping_add(SLOT2));
        let h2 = if s2 != 0 {
            lf_checker_rt::callee_thiscall!(4, u32, s2, pos);
            lf_checker_rt::callee_thiscall!(5, u32, voice_of(s2), base);
            rd32(s2.wrapping_add(HANDLE))
        } else {
            0xFFFF_FFFF
        };

        let mut buf = [p0, p1, p2];
        lf_checker_rt::callee_cdecl!(6, u32, h0, h1, h2, base, shaped, buf.as_mut_ptr() as u32);
        0
    }
});
