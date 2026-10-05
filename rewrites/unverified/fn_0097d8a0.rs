// original: 0x0097d8a0 audio_dual_voice_update (proposed)

/// Run the four-phase dual-voice update for the audio manager.
///
/// `this` is the audio manager object: dword at `+0x80` indexes the voice
/// table, dword at `+0x08` is carried into the request block, dword at
/// `+0x120` (plus `0x780`) is the bank base. `src` points at an object
/// whose dword at `+0x440` indexes the same table for the second voice;
/// `sel` (0..4, else return at once) indexes the two float tables.
///
/// Behaviour: the three global gates must pass or the function returns at
/// once, as must the source pointer and the selector range. Both table
/// entries must be non-null and supply voice ids from `+0x0A`. A request
/// block is built in frame scratch (block at frame `+0x10`): word 0 is a
/// randomised float plus table 1's entry (plus `3.0` when both ids are
/// equal), word 3 the bank base, word 8 the tag. Phase 1 issues
/// the first id (submit
/// triple `0,-1,0x59` on equal ids, `0,-1,0x5A` otherwise, or a slot
/// release); phase 2 issues the second id (triple `0,-1,0x5B`); phase 3
/// overwrites word 0 with table 2's entry and issues the global voice
/// (triple `0,-1,0x50`). Returns the last callee's answer.
///
/// Float order: `word0 = (mix1 + table1[sel]) + 3.0` on the equal path
/// (left-associated `addss` pair, written in that order), `word0 =
/// mix1 + table1[sel]` otherwise, `word2 = mix2 + table1[sel]`; all
/// other floats are copies. The gate-1 exit returns the caller's entry
/// EAX, which a rewrite cannot observe; the proof keeps gate 1 passing
/// and notes this, as well as the untested null-pointer exits. The
/// original stashes the second id in its incoming selector slot; that
/// store is above the frame, so the stack check is off and the value is
/// verified through the call logs instead.
///
/// Original: 0x0097D8A0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0097d8a0(this: u32, src: u32, sel: u32) -> u32 {
    unsafe {
        const GATE_INIT: u32 = 0x11F7060;
        const GATE_EPOCH_A: u32 = 0x12088B4;
        const GATE_EPOCH_B: u32 = 0xF1C040;
        const GATE_MODE: u32 = 0x1037720;
        const GATE_MODE_EXIT: u32 = 0x12;
        const VOICE_TABLE: u32 = 0x1231320;
        const VOICE_GLOBAL: u32 = 0x1231574;
        const RAND_F0: u32 = 0x1038934;
        const RAND_F1: u32 = 0x1038938;
        const FLOAT_TAB1: u32 = 0x103893C;
        const FLOAT_TAB2: u32 = 0x1038950;
        const EQUAL_BOOST: f32 = 3.0;
        const THIS_INDEX: u32 = 0x80;
        const THIS_TAG: u32 = 0x08;
        const THIS_X120: u32 = 0x120;
        const SRC_INDEX: u32 = 0x440;
        const BANK_BIAS: u32 = 0x780;
        const ENTRY_ID_OFF: u32 = 0x0A;
        const SEL_MAX: u32 = 5;
        const F_TRIPLE: usize = 0x04 / 4;
        const F_BLOCK: usize = 0x10 / 4;
        const BLOCK_BANK: usize = 0x0C / 4;
        const BLOCK_TAG: usize = 0x20 / 4;
        const C_BLOCK_INIT: u32 = 1;
        const C_RANDOMISE_A: u32 = 2;
        const C_RANDOMISE_B: u32 = 3;
        const C_SLOT_ALLOC: u32 = 4;
        const C_SLOT_RESOLVE: u32 = 5;
        const C_ISSUE: u32 = 6;
        const C_SUBMIT: u32 = 7;
        const C_RELEASE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        if lf_checker_rt::global::<u32>(GATE_INIT).read() == 1 {
            // Unreachable under the proof (gate pinned passing): the
            // original returns entry EAX here, which no rewrite can read.
            return 0;
        }
        let epoch = lf_checker_rt::global::<u32>(GATE_EPOCH_A).read();
        if epoch != lf_checker_rt::global::<u32>(GATE_EPOCH_B).read() {
            return epoch;
        }
        if lf_checker_rt::global::<u32>(GATE_MODE).read() == GATE_MODE_EXIT {
            return epoch;
        }
        if src == 0 {
            return epoch;
        }
        if sel >= SEL_MAX {
            return epoch;
        }
        let e1 = rd32(
            lf_checker_rt::relocated(VOICE_TABLE)
                .wrapping_add(rd32(this.wrapping_add(THIS_INDEX)).wrapping_mul(4)),
        );
        let e2 = rd32(
            lf_checker_rt::relocated(VOICE_TABLE)
                .wrapping_add(rd32(src.wrapping_add(SRC_INDEX)).wrapping_mul(4)),
        );
        if e1 == 0 || e2 == 0 {
            return 0;
        }
        let id1 = rd32(e1.wrapping_add(ENTRY_ID_OFF));
        let id2 = rd32(e2.wrapping_add(ENTRY_ID_OFF));
        let mut frame = [0u32; 0x80];
        let base = frame.as_mut_ptr() as u32;
        let block = base.wrapping_add((F_BLOCK * 4) as u32);
        lf_checker_rt::callee_thiscall!(C_BLOCK_INIT, u32, block);
        frame[F_BLOCK + BLOCK_TAG] = rd32(this.wrapping_add(THIS_TAG));
        core::hint::black_box(frame[F_BLOCK + BLOCK_TAG]);
        frame[F_BLOCK + BLOCK_BANK] =
            rd32(this.wrapping_add(THIS_X120)).wrapping_add(BANK_BIAS);
        let f0 = lf_checker_rt::global::<u32>(RAND_F0).read();
        let f1 = lf_checker_rt::global::<u32>(RAND_F1).read();
        let mix1: f32 = lf_checker_rt::callee_cdecl!(C_RANDOMISE_A, f32, f0, f1);
        let t1 = f32::from_bits(rd32(
            lf_checker_rt::relocated(FLOAT_TAB1).wrapping_add(sel.wrapping_mul(4)),
        ));
        // NOTE: keep the original's left-associated order: (mix + t) + 3.
        let w0 = mix1 + t1;
        frame[F_BLOCK] = if id1 == id2 { w0 + EQUAL_BOOST } else { w0 }.to_bits();
        let triple = base.wrapping_add((F_TRIPLE * 4) as u32);
        // Phase 1: the first voice id (submit id 0x59 on equal ids).
        // Equal ids skip phase 2 and continue at phase 3.
        let slot = lf_checker_rt::callee_cdecl!(C_SLOT_ALLOC, u32,);
        let resolved = lf_checker_rt::callee_cdecl!(C_SLOT_RESOLVE, u32, slot);
        let ok =
            lf_checker_rt::callee_thiscall!(C_ISSUE, u32, this, id1, block, slot, resolved, 0);
        if (ok as u8) == 0 {
            lf_checker_rt::callee_cdecl!(C_RELEASE, u32, slot);
        } else {
            frame[F_TRIPLE] = 0;
            frame[F_TRIPLE + 1] = 0xFFFF_FFFF;
            frame[F_TRIPLE + 2] = if id1 == id2 { 0x59 } else { 0x5A };
            lf_checker_rt::callee_cdecl!(
                C_SUBMIT, u32, id1, 0, 0, 1, block, triple,
                rd32(this.wrapping_add(THIS_X120)), slot
            );
        }
        // Phase 2: the second voice id (unequal ids only). Its mixed
        // float overwrites block word 0.
        if id1 != id2 {
            let mix2: f32 = lf_checker_rt::callee_cdecl!(C_RANDOMISE_B, f32, f0, f1);
            frame[F_BLOCK] = (mix2 + t1).to_bits();
            let slot2 = lf_checker_rt::callee_cdecl!(C_SLOT_ALLOC, u32,);
            let resolved2 = lf_checker_rt::callee_cdecl!(C_SLOT_RESOLVE, u32, slot2);
            let ok2 = lf_checker_rt::callee_thiscall!(
                C_ISSUE, u32, this, id2, block, slot2, resolved2, 0
            );
            if (ok2 as u8) == 0 {
                lf_checker_rt::callee_cdecl!(C_RELEASE, u32, slot2);
            } else {
                frame[F_TRIPLE] = 0;
                frame[F_TRIPLE + 1] = 0xFFFF_FFFF;
                frame[F_TRIPLE + 2] = 0x5B;
                lf_checker_rt::callee_cdecl!(
                    C_SUBMIT, u32, id2, 0, 0, 1, block, triple,
                    rd32(this.wrapping_add(THIS_X120)), slot2
                );
            }
        }
        // Phase 3: the global voice; block word 0 takes table 2's entry.
        let t2 = f32::from_bits(rd32(
            lf_checker_rt::relocated(FLOAT_TAB2).wrapping_add(sel.wrapping_mul(4)),
        ));
        frame[F_BLOCK] = t2.to_bits();
        let def = lf_checker_rt::global::<u32>(VOICE_GLOBAL).read();
        let slot3 = lf_checker_rt::callee_cdecl!(C_SLOT_ALLOC, u32,);
        let resolved3 = lf_checker_rt::callee_cdecl!(C_SLOT_RESOLVE, u32, slot3);
        let ok3 =
            lf_checker_rt::callee_thiscall!(C_ISSUE, u32, this, def, block, slot3, resolved3, 0);
        if (ok3 as u8) == 0 {
            return lf_checker_rt::callee_cdecl!(C_RELEASE, u32, slot3);
        }
        frame[F_TRIPLE] = 0;
        frame[F_TRIPLE + 1] = 0xFFFF_FFFF;
        frame[F_TRIPLE + 2] = 0x50;
        lf_checker_rt::callee_cdecl!(
            C_SUBMIT, u32, def, 0, 0, 1, block, triple,
            rd32(this.wrapping_add(THIS_X120)), slot3
        )
    }
});
