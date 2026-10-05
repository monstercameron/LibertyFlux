// original: 0x008c2660 frontend_state_poll_update (proposed)

/// Poll a status callback and advance a small frontend state machine.
///
/// `this` is forwarded untouched to the primary check callee and, with its
/// low byte replaced by a flag bit, as the argument of the second object
/// call; the function takes no stack arguments (thiscall, entry `(an instruction of the original)`
/// only reserves a scratch slot whose low byte the original reuses for
/// that flag). It returns the finaliser callee's answer, except on the
/// early path, where the original returns whatever `eax` held on entry:
/// the live id global when the first branch fell through, or the true
/// incoming `eax` when the mode global already selected the store path. A
/// rewrite cannot read incoming `eax`, so the contract pins it to zero
/// and the rewrite returns zero there (documented narrowing).
///
/// Behaviour in order: when the mode triple (mode, id pair, stage `0x12`)
/// selects the guarded path and the enable dword is set, either store the
/// marker `0x21` or take the early return. Otherwise run the primary check;
/// when it and the first poll both report quiet and either quiet byte is
/// clear, either clear three flag bytes or notify, depending on the same
/// mode triple. Poll twice more, folding each answer with the two quiet
/// bytes and two skip bytes into a go/no-go; a no-go runs a secondary
/// check and then the pair of object updates (first site) or the single
/// object update (last site). Between the polls, decode the state dword
/// (`2` and `6` are the live values, the latter indexed by a second
/// global through a four-entry table, anything else means slot 1) into a
/// slot id, a flag stored to the flag global, and a flag bit (set only
/// when the state is live and the index is zero) carried in the low byte
/// of the forwarded `this`; track the running signed minimum of the slot
/// id into one state global and push slot and flag through two object
/// calls. When the reentry byte is clear, run the next update stage
/// before the last poll.
///
/// Original: 0x008c2660 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_008c2660(this: u32) -> u32 {
    unsafe {
        const G_MODE: u32 = 0x011F7060;
        const G_ID_A: u32 = 0x012088B4;
        const G_ID_B: u32 = 0x00F1C040;
        const G_STAGE: u32 = 0x01037720;
        const STAGE_LIVE: u32 = 0x12;
        const G_ENABLE: u32 = 0x011F66A0;
        const B_GATE: u32 = 0x010376E8;
        const G_ARM: u32 = 0x010376EC;
        const G_MARKER: u32 = 0x0115A438;
        const MARKER: u32 = 0x21;
        const G_POLL_SLOT: u32 = 0x00E733DC;
        const G_POLL_ARG: u32 = 0x017ACCD8;
        const B_QUIET1: u32 = 0x0105B48F;
        const B_QUIET2: u32 = 0x017ED8D1;
        const B_SKIP1: u32 = 0x01173590;
        const B_SKIP2: u32 = 0x01173591;
        const G_STATE: u32 = 0x01030088;
        const G_INDEX: u32 = 0x01160D74;
        const G_SLOT_MIN: u32 = 0x01030090;
        const G_SLOT_LO: u32 = 0x0103008C;
        const G_FLAG: u32 = 0x01030094;
        const B_REENTRY: u32 = 0x01030C10;
        const G_FINAL_ARG: u32 = 0x01161904;
        const OBJ_MID: u32 = 0x01165880;
        const OBJ_WIDE: u32 = 0x012389E0;
        const OBJ_STATE: u32 = 0x0115DEF0;
        const OBJ_TAIL: u32 = 0x01231800;
        const C_PRIMARY: u32 = 2;
        const C_CLEAR: u32 = 3;
        const C_NOTIFY: u32 = 4;
        const C_SECOND_A: u32 = 5;
        const C_SECOND_B: u32 = 6;
        const C_MID: u32 = 7;
        const C_WIDE: u32 = 8;
        const C_STATE2: u32 = 9;
        const C_STATE1: u32 = 10;
        const C_NEXT_STAGE: u32 = 11;
        const C_TAIL: u32 = 12;
        const C_FINAL: u32 = 13;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(a) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(a) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn quiet_bytes_clear() -> bool {
            unsafe { rd8(B_QUIET1) == 0 || rd8(B_QUIET2) == 0 }
        }

        // Guarded store-or-early-return.
        if !(rd32(G_MODE) != 1 && rd32(G_ID_A) == rd32(G_ID_B) && rd32(G_STAGE) != STAGE_LIVE) {
            if rd32(G_ENABLE) != 0 {
                if rd8(B_GATE) != 0 {
                    wr32(G_MARKER, MARKER);
                } else if rd32(G_ARM) != 1 {
                    // Early return: the original returns entry eax here.
                    // Contract pins incoming eax to 0 (see doc comment).
                    if rd32(G_MODE) == 1 {
                        return 0;
                    }
                    return rd32(G_ID_A);
                } else {
                    wr32(G_MARKER, MARKER);
                }
            }
        }

        let poll: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(rd32(G_POLL_SLOT) as usize);
        let probe = rd32(G_POLL_ARG);

        // First poll: clear flags or notify.
        let primary = lf_checker_rt::callee_thiscall!(C_PRIMARY, u32, this);
        if (primary as u8) == 0 && poll(probe) == 0 && quiet_bytes_clear() {
            if rd32(G_MODE) != 1 && rd32(G_ID_A) == rd32(G_ID_B) && rd32(G_STAGE) != STAGE_LIVE
            {
                lf_checker_rt::callee_cdecl!(C_CLEAR, u32,);
            }
        } else {
            lf_checker_rt::callee_cdecl!(C_NOTIFY, u32,);
        }

        // Second poll with the object-update pair behind it.
        let round2 = poll(probe);
        let mut go = (round2 != 0 || !quiet_bytes_clear()) as u8;
        go |= rd8(B_SKIP1) | rd8(B_SKIP2);
        if go == 0 {
            let second = lf_checker_rt::callee_cdecl!(C_SECOND_A, u32,);
            if (second as u8) == 0 {
                lf_checker_rt::callee_thiscall!(C_MID, u32, lf_checker_rt::relocated(OBJ_MID));
                lf_checker_rt::callee_thiscall!(C_WIDE, u32, lf_checker_rt::relocated(OBJ_WIDE));
            }
        }

        // Slot/flag decode and state tracking. The flag bit rides in the
        // low byte of the forwarded object pointer, not in the flag global.
        let state = rd32(G_STATE);
        let index = rd32(G_INDEX);
        let (slot, flag) = match state {
            2 => (1u32, 0u32),
            6 => match index {
                0 => (1, 0),
                1 => (1, 0),
                2 => (3, 1),
                3 => (3, 0),
                _ => (3, 1),
            },
            _ => (1, 0),
        };
        let flag_bit = ((state == 2 || state == 6) && index == 0) as u32;
        if slot != rd32(G_SLOT_MIN) {
            let lo = rd32(G_SLOT_LO);
            wr32(G_SLOT_MIN, (slot as i32).min(lo as i32) as u32);
        }
        wr32(G_FLAG, flag);
        let obj_state = lf_checker_rt::relocated(OBJ_STATE);
        lf_checker_rt::callee_thiscall!(C_STATE2, u32, obj_state, slot, flag);
        lf_checker_rt::callee_thiscall!(C_STATE1, u32, obj_state, (this & 0xFFFF_FF00) | flag_bit);

        // Next stage unless reentered.
        if rd8(B_REENTRY) == 0 {
            lf_checker_rt::callee_cdecl!(C_NEXT_STAGE, u32,);
        }

        // Last poll with the single object update behind it.
        let round3 = poll(probe);
        let mut go3 = (round3 != 0 || !quiet_bytes_clear()) as u8;
        go3 |= rd8(B_SKIP1) | rd8(B_SKIP2);
        if go3 == 0 {
            let second = lf_checker_rt::callee_cdecl!(C_SECOND_B, u32,);
            if (second as u8) == 0 {
                lf_checker_rt::callee_thiscall!(C_TAIL, u32, lf_checker_rt::relocated(OBJ_TAIL));
            }
        }

        lf_checker_rt::callee_cdecl!(C_FINAL, u32, rd32(G_FINAL_ARG))
    }
});
