// original: 0xb60870 audio_voice_setup
/// Audio voice/event setup: resolves an output handle for `this`, probes an
/// optional entity (`ent`), derives a position triple and a parameter vector
/// from it (or from the fallback buffers), and dispatches to the mixer
/// (`c10`) and post step (`c11`). Always returns 1.
///
/// Signature: thiscall, 6 stack args; the 5th (`_unused`) is never read.
/// Float constants come from the image (0.67-ish lift, 1.0 drop).
/// Two stack-grown value blocks in the original are dead stores (initialized
/// but never passed to any callee or returned); they are not reproduced.
export!(thiscall, rw_b77_f1(this: u32, ent: u32, buf_a: u32, buf_b: u32, tag: u32, _unused: u32, tgt: u32) -> u8 {
    unsafe {
        // c1: resolve output object from this+0x18; pick channel word.
        let out = callee_cdecl!(1, u32, *((this).wrapping_add(0x18) as *const u32));
        let mut chan = *((out).wrapping_add(0xa8) as *const u32);
        if chan == 0 {
            chan = *((this).wrapping_add(0x18) as *const u32);
        }
        // Null entity: forward straight to the mixer + post step.
        if ent == 0 {
            callee_cdecl!(10, u32, ent, chan, buf_a, buf_b, tag, 0, 0, tgt);
            callee_stdcall!(11, u32, ent, buf_a, 0, tag, 0);
            return 1;
        }
        // Optional target flag + saved target slot.
        let mut flag: u32 = 0;
        let mut saved_tgt: u32 = 0;
        if tgt != 0 {
            let mode = (*((tgt.wrapping_add(0x1e2)) as *const u8)) & 0x0f;
            if mode < 2 {
                saved_tgt = tgt;
                flag = 1;
            }
        }
        // c2: indirect entity probe through the vtable slot at +0xb8
        // (thiscall/4: entity, kind 0x24, &flag, &saved_tgt).
        let vtab = *((ent).wrapping_add(0) as *const u32);
        let probe: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*((vtab).wrapping_add(0xb8) as *const u32) as usize);
        probe(ent, &saved_tgt as *const u32 as u32, &flag as *const u32 as u32, 0x24, ent);
        // Position triple: strided data when present, else inline fields.
        let ext = *((ent).wrapping_add(0x20) as *const u32);
        let pbase = if ext != 0 { ext.wrapping_add(0x30) } else { ent.wrapping_add(0x10) };
        let mut pos = [*((pbase).wrapping_add(0) as *const f32), *((pbase).wrapping_add(4) as *const f32), *((pbase).wrapping_add(8) as *const f32)];
        // Entity kind check selects a validation pair.
        let mut hot: u8 = 0;
        if *((ent).wrapping_add(0x28) as *const u32) & 0x3c0 == 0x0c0 {
            // The probed flag byte is unaligned (flag<<8 shares the word);
            // only the low byte (always 0 here) feeds `hot`.
            let mut inner: u32 = 0;
            let packed: u32 = flag << 8;
            let ok = callee_thiscall!(3, u8, *((ent).wrapping_add(0x224) as *const u32), &inner as *const u32 as u32, &packed as *const u32 as u32);
            if ok != 0 {
                hot = 1;
            } else {
                let ok2 = callee_thiscall!(4, u8, ent);
                if ok2 == 0 {
                    pos[2] += *global::<f32>(0xeb1170);
                }
            }
        }
        // Parameter vector: target buffer when aimed, else fallback buffer.
        let vec4: [f32; 4];
        if tgt != 0 {
            pos[2] = *((buf_a).wrapping_add(0x38) as *const f32);
            vec4 = [*((buf_a).wrapping_add(0x30) as *const f32), *((buf_a).wrapping_add(0x34) as *const f32), *((buf_a).wrapping_add(0x38) as *const f32), *((buf_a).wrapping_add(0x3c) as *const f32)];
        } else {
            vec4 = [*((buf_b).wrapping_add(0) as *const f32), *((buf_b).wrapping_add(4) as *const f32), *((buf_b).wrapping_add(8) as *const f32), *((buf_b).wrapping_add(12) as *const f32)];
        }
        if hot == 0 {
            // c5: nine-word validation over the scratch values.
            let mut zero: u32 = 0;
            let accepted = callee_cdecl!(5, u32,
                pos.as_ptr() as u32,
                vec4.as_ptr() as u32,
                &saved_tgt as *const u32 as u32,
                &zero as *const u32 as u32,
                0x8e, 1, flag, 0, 4);
            if accepted != 0 {
                match *((out).wrapping_add(0xa0) as *const u32) {
                    // Direct channel: configure (c6) then mix (c10).
                    4 => {
                        let mut cfg: u32 = 0;
                        callee_thiscall!(6, u32, &cfg as *const u32 as u32, buf_a);
                        callee_cdecl!(10, u32, ent, chan, &cfg as *const u32 as u32, buf_b, tag, 0, 0, tgt);
                    }
                    // Entity channels: rebuild the triple, maybe attach (c8/c9), then mix.
                    0 | 1 => {
                        let q = *((ent).wrapping_add(0x20) as *const u32);
                        let qb = if q != 0 { q.wrapping_add(0x30) } else { ent.wrapping_add(0x10) };
                        // Two copies: the plain triple and one with the third element dropped by 1.0.
                        let t = [*((qb).wrapping_add(0) as *const f32), *((qb).wrapping_add(4) as *const f32), *((qb).wrapping_add(8) as *const f32)];
                        let t_drop = [t[0], t[1], t[2] - *global::<f32>(0xfe88e8)];
                        if q == 0 {
                            callee_thiscall!(8, u32, ent);
                            callee_thiscall!(9, u32, ent.wrapping_add(0x10), q);
                        }
                        callee_cdecl!(10, u32, ent, chan, q, t.as_ptr() as u32, t_drop.as_ptr() as u32, 0, 0, tgt);
                    }
                    // Anything else: side path (c7), mixer skipped.
                    _ => {
                        callee_cdecl!(7, u32, ent, chan, buf_b);
                    }
                }
                callee_stdcall!(11, u32, ent, buf_a, 0, tag, 0);
                return 1;
            }
        }
        // Cold path: forward to the mixer + post step.
        callee_cdecl!(10, u32, ent, chan, buf_a, buf_b, tag, 0, 0, tgt);
        callee_stdcall!(11, u32, ent, buf_a, 0, tag, 0);
        1
    }
});
