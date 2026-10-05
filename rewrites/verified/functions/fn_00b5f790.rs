// original: 0x00b5f790 audio_voice_gate (proposed)

/// Gate an audio voice update on entity flags, stats and a level check.
///
/// `this` is the voice controller (`+0x18` is the key passed to the info and
/// notify callees). `arg0` is a candidate entity: it is kept (as `kept`) only
/// when non-null, its flag word `+0x28` masked with `0x3C0` equals `0xC0`,
/// and its byte `+0x219` is non-zero. `arg1` is never read. `arg2` points at
/// a dword handed to the entity resolver; `arg3` is an f32 level.
///
/// Behaviour: resolve the entity for `[arg2]`; when the info record for the
/// key has bit 5 of `+0x20` set and the entity is live, run the mixer callee
/// with tag 1, then — if the entity still validates (`+0x28` mask `0xC0`,
/// `+0x210` non-zero implies `+0xA74` is neither 1 nor 2, `+0x224` non-zero)
/// — run it again with tag `0xE` or `0xD` depending on whether
/// `[[entity+0x21C]+0x12C]` equals 2. When the level is above zero (ordered)
/// and the entity validates, increment the use count behind
/// `[[kept+0x228]+0x54C]`. Then, unless `kept` is null, the entity is null,
/// or both are the same object, require the info record's `+0xC` to equal 2.
/// When the entity kind `([esi+0x28] >> 6) & 0xF` is 2, 3 or 4, compare two
/// float stats (ids `0x120`/`0x11F`); if the second is not below the first
/// (unordered counts as below) and the stamp global differs from its shadow,
/// copy the stamp and add 1.0 to stat `0x120`. Pick the level object as
/// `[kept+0x398]` or the entity when that is null; run its virtual slot
/// `0xFC` (f32 level) according to its `+0x28` mask (`0xC0`: flag=1 when the
/// level is above zero; `0x80`: same unless `+0x28 & 0x7C00 == 0xC00`;
/// `0x100`: flag = level above zero); clear the flag when dword `+0x118`
/// has bit `0x400` set, and in the `0x80` case also when byte `+0x118` has
/// bit `0x40` set (the original falls through from the byte test into the
/// dword test, so either bit clears); and clear it when the `0xC0`-case gate
/// callee returns a non-zero low byte. Finally, when the level object is the
/// entity itself and the flag is set, run the notify callee with the key.
///
/// All integer comparisons are equality or bit tests (no signed relational
/// comparison of a callee-returned value). All float comparisons are ordered
/// (`>` / `>=`); unordered operands take the same side as below-or-equal, as
/// the original's `comiss`+`jb`/`jbe` does. No floating-point arithmetic.
///
/// Original: 0x00B5F790 (thiscall, four stack words; the second is unread;
/// no return value).
lf_checker_rt::export!(thiscall, rw_00b5f790(
    this: u32, arg0: u32, _arg1: u32, arg2: u32, arg3: u32
) -> u32 {
    unsafe {
        const KIND_MASK: u32 = 0x3C0;
        const KIND_LIVE: u32 = 0xC0;
        const KIND_QUIET: u32 = 0x80;
        const KIND_PLAIN: u32 = 0x100;
        const THIS_KEY: u32 = 0x18;
        const ENT_FLAGS: u32 = 0x28;
        const ENT_EXTRA: u32 = 0x118;
        const ENT_ALIVE_B: u32 = 0x210;
        const ENT_MODE: u32 = 0xA74;
        const ENT_SLOT: u32 = 0x224;
        const ENT_TAB: u32 = 0x21C;
        const ENT_LINK: u32 = 0x219;
        const ENT_USEP: u32 = 0x228;
        const ENT_ALT: u32 = 0x398;
        const TAB_TAG: u32 = 0x12C;
        const USE_COUNT: u32 = 0x54C;
        const VTABLE_LEVEL: u32 = 0xFC;
        const INFO_BIT: u32 = 0x20;
        const INFO_KIND: u32 = 0x0C;
        const STAT_A: u32 = 0x120;
        const STAT_B: u32 = 0x11F;
        const TAG_FIRST: u32 = 1;
        const TAG_HIT: u32 = 0x0E;
        const TAG_MISS: u32 = 0x0D;
        const STAMP: u32 = 0x1173604;
        const STAMP_SHADOW: u32 = 0x1670534;
        const ID_RESOLVE: u32 = 1;
        const ID_INFO: u32 = 2;
        const ID_MIXER: u32 = 3;
        const ID_STAT_A: u32 = 4;
        const ID_STAT_B: u32 = 5;
        const ID_STAT_ADD: u32 = 6;
        const ID_GATE2: u32 = 8;
        const ID_NOTIFY: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        let live = |e: u32| unsafe { rd32(e + ENT_FLAGS) } & KIND_MASK == KIND_LIVE;

        // Candidate gate: null, flag mask, link byte.
        let mut kept = 0u32;
        if arg0 != 0 && live(arg0) && rd8(arg0 + ENT_LINK) != 0 {
            kept = arg0;
        }

        let key = rd32(this + THIS_KEY);
        let entity: u32 = lf_checker_rt::callee_cdecl!(ID_RESOLVE, u32, rd32(arg2));
        let info: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);

        // Mixer setup block.
        if (rd32(info + INFO_BIT) >> 5) & 1 != 0 && entity != 0 {
            lf_checker_rt::callee_cdecl!(ID_MIXER, u32, TAG_FIRST, entity, kept);
            if live(entity)
                && (rd8(entity + ENT_ALIVE_B) == 0
                    || { let m = rd32(entity + ENT_MODE); m != 1 && m != 2 })
                && rd32(entity + ENT_SLOT) != 0
            {
                let tag = if rd32(rd32(entity + ENT_TAB) + TAB_TAG) == 2 {
                    TAG_HIT
                } else {
                    TAG_MISS
                };
                lf_checker_rt::callee_cdecl!(ID_MIXER, u32, tag, entity, kept);
            }
        }

        // Level gate: ordered above-zero float compare, then the use count.
        let level = f32::from_bits(arg3);
        if level > 0.0 && entity != 0 && live(entity) {
            let slot = rd32(kept + ENT_USEP);
            let count = rd32(slot + USE_COUNT);
            (slot.wrapping_add(USE_COUNT) as *mut u32).write_unaligned(count.wrapping_add(1));
        }

        if kept == 0 || entity == 0 || entity == kept {
            return 0;
        }
        let info2: u32 = lf_checker_rt::callee_cdecl!(ID_INFO, u32, key);
        if rd32(info2 + INFO_KIND) != 2 {
            return 0;
        }

        // Stats block for entity kinds 2, 3, 4.
        let kind = (rd32(entity + ENT_FLAGS) >> 6) & 0xF;
        if kind == 2 || kind == 3 || kind == 4 {
            let stat_a: f32 = lf_checker_rt::callee_cdecl!(ID_STAT_A, f32, STAT_A);
            let stat_b: f32 = lf_checker_rt::callee_cdecl!(ID_STAT_B, f32, STAT_B);
            // Original `comiss`+`jb`: unordered counts as below.
            if stat_b >= stat_a {
                let stamp = (lf_checker_rt::global::<u32>(STAMP) as *const u32).read_unaligned();
                let shadow =
                    (lf_checker_rt::global::<u32>(STAMP_SHADOW) as *const u32).read_unaligned();
                if stamp != shadow {
                    (lf_checker_rt::global::<u32>(STAMP_SHADOW) as *mut u32)
                        .write_unaligned(stamp);
                    lf_checker_rt::callee_cdecl!(ID_STAT_ADD, u32, STAT_A, 1.0f32.to_bits());
                }
            }
        }

        // Level object + flag.
        let alt = rd32(kept + ENT_ALT);
        let lvl = if alt != 0 { alt } else { entity };
        let mut flag = 0u32;
        if lvl != 0 {
            let read_level = |o: u32| unsafe {
                let f: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(rd32(rd32(o) + VTABLE_LEVEL) as usize);
                f(o)
            };
            match rd32(lvl + ENT_FLAGS) & KIND_MASK {
                KIND_LIVE => {
                    if read_level(lvl) > 0.0 {
                        flag = 1;
                    }
                }
                KIND_QUIET => {
                    if read_level(lvl) > 0.0 && rd32(lvl + ENT_FLAGS) & 0x7C00 != 0xC00 {
                        flag = 1;
                    }
                }
                KIND_PLAIN => {
                    if read_level(lvl) > 0.0 {
                        flag = 1;
                    }
                }
                _ => {}
            }
            // The original falls through from the byte test to the dword
            // test, so either bit clears the flag in the 0x80 case.
            if rd32(lvl + ENT_FLAGS) & KIND_MASK == KIND_QUIET
                && rd8(lvl + ENT_EXTRA) & 0x40 != 0
            {
                flag = 0;
            }
            if rd32(lvl + ENT_EXTRA) & 0x400 != 0 {
                flag = 0;
            }
            if rd32(lvl + ENT_FLAGS) & KIND_MASK == KIND_LIVE {
                let ok: u32 = lf_checker_rt::callee_cdecl!(ID_GATE2, u32, lvl, kept);
                if ok as u8 != 0 {
                    flag = 0;
                }
            }
            if lvl == entity && flag != 0 {
                lf_checker_rt::callee_cdecl!(ID_NOTIFY, u32, key);
            }
        }
        0
    }
});
