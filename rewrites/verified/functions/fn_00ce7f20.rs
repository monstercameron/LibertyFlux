// original: 0x00CE7F20 CTaskSimpleNMBrace::vf26 (merged symbol, vtable slot 26)

/// Brace-task per-frame update: report the brace parameters, run the gated
/// aim/attach step, send a one- or two-message NaturalMotion update, then run
/// the ped animation/emote tail.
///
/// `this` is the task, `ped` the ped. The task word at `+0x58` is a type
/// index selecting one row (stride `0x54`) of a parameter table; the same
/// index also selects a word of a second table for the opening log call.
///
/// Stages, in order:
/// 1. Log call with the format string, the type index and the type-table
///    word.
/// 2. First message: constructor, one boolean slot, then float slots filled
///    from the parameter row (`+0x00`, `+0x04`, `+0x08`, a byte at `+0x4c`,
///    `+0x0c`, `+0x10`, `+0x14`), one interpolated slot
///    `(row+0x1c - row+0x18) * (rand() as float * scale) + row+0x18`, one
///    more float (`+0x44`), then send through the ped's context (`+0x7b4`).
///    (The interpolated slot blends toward `+0x18`: see the note at the
///    computation.)
/// 3. Gate block, only when the task byte at `+0x50` is clear: a sub-call on
///    `this+0x28` must answer nonzero, then four row dwords (`+0x20`..`+0x2c`)
///    are staged; the task link at `+0x54` must exist with mode bits
///    (`+0x28` masked with `0x3c0`) equal to `0x80` and a non-null aux block
///    at `+0xf50`; an anchor query on the ped word at `+0x224` must answer
///    nonzero while the task word at `+0x4c` is zero; then the staged words
///    are handed to the parameter callee with 2 when the ped byte at `+0x219`
///    is clear, else 1.
/// 4. Second branch on bit 0 of the ped byte at `+0x14f`: when set, reset
///    the first message, set one float to 0.3 and send; otherwise build a
///    second message (boolean, one float from the word just before the row,
///    send), then reset/set/send the first message with the row float at
///    `+0x48`, and destroy the second message.
/// 5. Tail: a virtual call through the task vtable (slot `0x6c`), one of two
///    ped animation calls chosen by `rand() < 0x3fff` (signed), and, when the
///    ped byte at `+0xa60` is 2, an emote chain of up to three calls on
///    `ped+0x570` (ten arguments, first nonzero answer wins) using one of
///    two string triples selected by an unsigned `< 3` test on a byte
///    reached through the emote table indexed by the sign-extended ped word
///    at `+0x2e`.
/// 6. Destroy the first message and run the security-cookie check.
///
/// The only float arithmetic is the stage-2 interpolation, in the original's
/// operation order. Returns the last callee's answer. Original: 0x00CE7F20
/// (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ce7f20(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_TYPE: u32 = 0x58;
        const TASK_SUB: u32 = 0x28;
        const TASK_LINK: u32 = 0x54;
        const TASK_FLAG_4C: u32 = 0x4c;
        const TASK_FLAG_50: u32 = 0x50;
        const PED_NMCTX: u32 = 0x7b4;
        const PED_QOBJ: u32 = 0x224;
        const PED_FLAG_219: u32 = 0x219;
        const PED_FLAG_14F: u32 = 0x14f;
        const PED_STATE_A60: u32 = 0xa60;
        const PED_EMOTE_IDX: u32 = 0x2e;
        const LINK_MODE: u32 = 0x28;
        const LINK_AUX: u32 = 0xf50;
        const MODE_MASK: u32 = 0x3c0;
        const MODE_BRACE: u32 = 0x80;
        const VT_SLOT: u32 = 0x6c;
        const ROW_BASE: u32 = 0x171d6c4;
        const ROW_STRIDE: u32 = 0x54;
        const TYPE_TABLE: u32 = 0x10521bc;
        const EMOTE_TABLE: u32 = 0x1295cd8;
        const LOG_FMT: u32 = 0xedb8d4;
        const RAND_SCALE: u32 = 0xfe8684;
        const NAME_ENABLE: u32 = 0x1051cc8;
        const NAME_F0: u32 = 0x1051edc;
        const NAME_F1: u32 = 0x1051ee0;
        const NAME_F2: u32 = 0x1051ee4;
        const NAME_B0: u32 = 0x1051eec;
        const NAME_F3: u32 = 0x1051ef0;
        const NAME_F4: u32 = 0x1051ef4;
        const NAME_F5: u32 = 0x1051ef8;
        const NAME_LERP: u32 = 0x1051efc;
        const NAME_F6: u32 = 0x1051f00;
        const NAME_SEND_A: u32 = 0x1051ec8;
        const NAME_F7: u32 = 0x1051d10;
        const NAME_SEND_B: u32 = 0x1051cf8;
        const NAME_BF0: u32 = 0x10520f4;
        const NAME_BSEND: u32 = 0x10520ec;
        const STR_HI0: u32 = 0xedb9c8;
        const STR_HI1: u32 = 0xedb9f4;
        const STR_HI2: u32 = 0xedba18;
        const STR_LO0: u32 = 0xedba60;
        const STR_LO1: u32 = 0xedba94;
        const STR_LO2: u32 = 0xedbab8;
        const F_ONE: u32 = 0x3f800000;
        const F_POINT3: u32 = 0x3e99999a;
        const RAND_SPLIT: i32 = 0x3fff;
        const LOGMSG: u32 = 1;
        const NM_CTOR: u32 = 2;
        const NM_SETBOOL: u32 = 3;
        const NM_SETFLOAT: u32 = 4;
        const RAND: u32 = 5;
        const NM_SEND: u32 = 6;
        const GATE_SUB: u32 = 7;
        const FRAME_INIT: u32 = 8;
        const ANCHOR_Q: u32 = 9;
        const PARAM_COPY: u32 = 10;
        const NM_RESET: u32 = 11;
        const PED_ANIM_A: u32 = 13;
        const PED_ANIM_B: u32 = 14;
        const PED_EMOTE: u32 = 15;
        const NM_DTOR: u32 = 16;
        const COOKIE: u32 = 17;

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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gb(va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(va)) }
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

        let mut eax = 0u32;
        let ty = rd32(this + TASK_TYPE);
        let row = ROW_BASE.wrapping_add(ty.wrapping_mul(ROW_STRIDE));

        // Stage 1: log call.
        eax = lf_checker_rt::callee_cdecl!(
            LOGMSG, u32,
            lf_checker_rt::relocated(LOG_FMT),
            ty,
            g32(TYPE_TABLE.wrapping_add(ty.wrapping_mul(4)))
        );

        // Stage 2: first message.
        let mut msg_a = [0u32; 16];
        let buf_a = msg_a.as_mut_ptr() as u32;
        let mut msg_b = [0u32; 16];
        let buf_b = msg_b.as_mut_ptr() as u32;
        eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf_a);
        eax = lf_checker_rt::callee_thiscall!(NM_SETBOOL, u32, buf_a, g32(NAME_ENABLE), 1);
        eax = lf_checker_rt::callee_thiscall!(NM_SETFLOAT, u32, buf_a, g32(NAME_F0), g32(row));
        eax = lf_checker_rt::callee_thiscall!(
            NM_SETFLOAT, u32, buf_a, g32(NAME_F1), g32(row.wrapping_add(4)));
        eax = lf_checker_rt::callee_thiscall!(
            NM_SETFLOAT, u32, buf_a, g32(NAME_F2), g32(row.wrapping_add(8)));
        eax = lf_checker_rt::callee_thiscall!(
            NM_SETBOOL, u32, buf_a, g32(NAME_B0), gb(row.wrapping_add(0x4c)) as u32);
        eax = lf_checker_rt::callee_thiscall!(
            NM_SETFLOAT, u32, buf_a, g32(NAME_F3), g32(row.wrapping_add(0x0c)));
        eax = lf_checker_rt::callee_thiscall!(
            NM_SETFLOAT, u32, buf_a, g32(NAME_F4), g32(row.wrapping_add(0x10)));
        eax = lf_checker_rt::callee_thiscall!(
            NM_SETFLOAT, u32, buf_a, g32(NAME_F5), g32(row.wrapping_add(0x14)));
        let r = lf_checker_rt::callee_cdecl!(RAND, u32,);
        eax = r;
        let scale = mul(
            core::hint::black_box(r as i32) as f32,
            f32::from_bits(g32(RAND_SCALE)),
        );
        let rb = f32::from_bits(g32(row.wrapping_add(0x18)));
        let rc = f32::from_bits(g32(row.wrapping_add(0x1c)));
        // Note: a push between the sub and the add shifts esp, so the final
        // addend is [esp0+8] (b), not [esp0+0xc]: v = (c-b)*f + b.
        let v = add(mul(sub(rc, rb), scale), rb);
        eax = lf_checker_rt::callee_thiscall!(
            NM_SETFLOAT, u32, buf_a, g32(NAME_LERP), v.to_bits());
        eax = lf_checker_rt::callee_thiscall!(
            NM_SETFLOAT, u32, buf_a, g32(NAME_F6), g32(row.wrapping_add(0x44)));
        let nmctx = rd32(ped + PED_NMCTX);
        eax = lf_checker_rt::callee_thiscall!(NM_SEND, u32, nmctx, g32(NAME_SEND_A), buf_a);

        // Stage 3: gate block.
        if rd8(this + TASK_FLAG_50) == 0 {
            let g = lf_checker_rt::callee_thiscall!(
                GATE_SUB, u32, this.wrapping_add(TASK_SUB), ped);
            eax = g;
            if (g as u8) != 0 {
                let mut staged = [0u32; 4];
                let staged_p = staged.as_mut_ptr() as u32;
                eax = lf_checker_rt::callee_thiscall!(FRAME_INIT, u32, staged_p);
                staged[0] = g32(row.wrapping_add(0x20));
                staged[1] = g32(row.wrapping_add(0x24));
                staged[2] = g32(row.wrapping_add(0x28));
                staged[3] = g32(row.wrapping_add(0x2c));
                let link = rd32(this + TASK_LINK);
                if link != 0 && (rd32(link + LINK_MODE) & MODE_MASK) == MODE_BRACE {
                    let aux = rd32(link + LINK_AUX);
                    if aux != 0 {
                        let q = lf_checker_rt::callee_thiscall!(
                            ANCHOR_Q, u32, rd32(ped + PED_QOBJ), aux, 1);
                        eax = q;
                        if (q as u8) != 0 && rd32(this + TASK_FLAG_4C) == 0 {
                            let n = if rd8(ped + PED_FLAG_219) == 0 { 2 } else { 1 };
                            eax = lf_checker_rt::callee_thiscall!(
                                PARAM_COPY, u32, this.wrapping_add(TASK_SUB), n, staged_p);
                        }
                    }
                }
            }
        }

        // Stage 4: second branch.
        if rd8(ped + PED_FLAG_14F) & 1 != 0 {
            eax = lf_checker_rt::callee_thiscall!(NM_RESET, u32, buf_a);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_a, g32(NAME_F7), F_POINT3);
            eax = lf_checker_rt::callee_thiscall!(NM_SEND, u32, nmctx, g32(NAME_SEND_B), buf_a);
        } else {
            eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf_b);
            eax = lf_checker_rt::callee_thiscall!(NM_SETBOOL, u32, buf_b, g32(NAME_ENABLE), 1);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_b, g32(NAME_BF0), g32(row.wrapping_sub(4)));
            eax = lf_checker_rt::callee_thiscall!(NM_SEND, u32, nmctx, g32(NAME_BSEND), buf_b);
            eax = lf_checker_rt::callee_thiscall!(NM_RESET, u32, buf_a);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_a, g32(NAME_F7), g32(row.wrapping_add(0x48)));
            eax = lf_checker_rt::callee_thiscall!(NM_SEND, u32, nmctx, g32(NAME_SEND_B), buf_a);
            eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf_b);
        }

        // Stage 5: tail.
        let vt = rd32(this);
        let vslot: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt + VT_SLOT) as usize) };
        eax = vslot(this, ped);
        let r2 = lf_checker_rt::callee_cdecl!(RAND, u32,);
        eax = r2;
        if (r2 as i32) < RAND_SPLIT {
            eax = lf_checker_rt::callee_thiscall!(PED_ANIM_A, u32, ped);
        } else {
            eax = lf_checker_rt::callee_thiscall!(PED_ANIM_B, u32, ped, 0xFFFF_FFFF);
        }
        if rd8(ped + PED_STATE_A60) == 2 {
            let idx = rd16(ped + PED_EMOTE_IDX) as u16 as i16 as i32 as u32;
            let e = g32(EMOTE_TABLE.wrapping_add(idx.wrapping_mul(4)));
            let triple = if rd8(e.wrapping_add(0xee)) < 3 {
                [STR_LO0, STR_LO1, STR_LO2]
            } else {
                [STR_HI0, STR_HI1, STR_HI2]
            };
            let ethis = ped.wrapping_add(0x570);
            for s in triple {
                let a = lf_checker_rt::callee_thiscall!(
                    PED_EMOTE, u32, ethis, lf_checker_rt::relocated(s),
                    0, 0, 0, 0xFFFF_FFFF, 0, 0, F_ONE, 0, 0);
                eax = a;
                if (a as u8) != 0 {
                    break;
                }
            }
        }

        // Stage 6: destroy the first message, cookie check.
        eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf_a);
        lf_checker_rt::callee_stdcall!(COOKIE, u32,);
        eax
    }
});
