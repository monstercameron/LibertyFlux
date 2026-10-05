// original: 0x0097dba0 audio_default_voice_update (proposed)

/// Run the default-voice two-phase update for the audio manager.
///
/// `this` is the audio manager object: dword at `+0x80` indexes the voice
/// table (global), dword at `+0x08` is carried into the request block, dword
/// at `+0x120` (plus `0x780`) is the bank base. Takes no stack arguments.
///
/// Behaviour: the same three global gates as its siblings must pass or the
/// function returns at once. The voice entry `table[[this+0x80]]` supplies
/// a word at `+0x0A` used as the phase-1 voice id. A request block is built
/// in frame scratch (block at frame `+0x18`): word 0 is a randomised float
/// from two global floats, word 3 the bank base, word 8 the `+0x08` tag.
/// Phase 1 issues the request for that voice id and either submits it
/// (triple `0,-1,0x5C` beside the block) or releases its slot; phase 2
/// zeroes block word 0 and repeats the issue/submit-or-release for the
/// global default voice id (triple `0,-1,0x57`). Returns the last callee's
/// answer.
///
/// Floats from globals are passed through untouched (bit copies); the only
/// float produced comes from the scripted randomiser callee. The gate-1
/// exit returns the caller's entry EAX, which a rewrite cannot observe; the
/// proof keeps gate 1 passing and notes this.
///
/// Original: 0x0097DBA0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0097dba0(this: u32) -> u32 {
    unsafe {
        const GATE_INIT: u32 = 0x11F7060;
        const GATE_EPOCH_A: u32 = 0x12088B4;
        const GATE_EPOCH_B: u32 = 0xF1C040;
        const GATE_MODE: u32 = 0x1037720;
        const GATE_MODE_EXIT: u32 = 0x12;
        const VOICE_TABLE: u32 = 0x1231320;
        const DEFAULT_VOICE: u32 = 0x12314A0;
        const RAND_F0: u32 = 0x1038934;
        const RAND_F1: u32 = 0x1038938;
        const THIS_INDEX: u32 = 0x80;
        const THIS_TAG: u32 = 0x08;
        const THIS_X120: u32 = 0x120;
        const BANK_BIAS: u32 = 0x780;
        const ENTRY_ID_OFF: u32 = 0x0A;
        const PHASE1_ID: u32 = 0x5C;
        const PHASE2_ID: u32 = 0x57;
        const F_TRIPLE: usize = 0x0C / 4;
        const F_BLOCK: usize = 0x18 / 4;
        const BLOCK_BANK: usize = 0x0C / 4;
        const BLOCK_TAG: usize = 0x20 / 4;
        const C_BLOCK_INIT: u32 = 1;
        const C_RANDOMISE: u32 = 2;
        const C_SLOT_ALLOC: u32 = 3;
        const C_SLOT_RESOLVE: u32 = 4;
        const C_ISSUE: u32 = 5;
        const C_SUBMIT: u32 = 6;
        const C_RELEASE: u32 = 7;

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
        let index = rd32(this.wrapping_add(THIS_INDEX));
        let entry = rd32(
            lf_checker_rt::relocated(VOICE_TABLE).wrapping_add(index.wrapping_mul(4)),
        );
        let voice = rd32(entry.wrapping_add(ENTRY_ID_OFF));
        let mut frame = [0u32; 0x80];
        let base = frame.as_mut_ptr() as u32;
        let block = base.wrapping_add((F_BLOCK * 4) as u32);
        lf_checker_rt::callee_thiscall!(C_BLOCK_INIT, u32, block);
        let f0 = lf_checker_rt::global::<u32>(RAND_F0).read();
        let f1 = lf_checker_rt::global::<u32>(RAND_F1).read();
        let mixed: f32 = lf_checker_rt::callee_cdecl!(C_RANDOMISE, f32, f0, f1);
        frame[F_BLOCK] = mixed.to_bits();
        frame[F_BLOCK + BLOCK_BANK] =
            rd32(this.wrapping_add(THIS_X120)).wrapping_add(BANK_BIAS);
        frame[F_BLOCK + BLOCK_TAG] = rd32(this.wrapping_add(THIS_TAG));
        let triple = base.wrapping_add((F_TRIPLE * 4) as u32);
        // Phase 1: the table voice id.
        let slot = lf_checker_rt::callee_cdecl!(C_SLOT_ALLOC, u32,);
        let resolved = lf_checker_rt::callee_cdecl!(C_SLOT_RESOLVE, u32, slot);
        let ok = lf_checker_rt::callee_thiscall!(C_ISSUE, u32, this, voice, block, slot, resolved, 0);
        if (ok as u8) == 0 {
            lf_checker_rt::callee_cdecl!(C_RELEASE, u32, slot);
        } else {
            frame[F_TRIPLE] = 0;
            frame[F_TRIPLE + 1] = 0xFFFF_FFFF;
            frame[F_TRIPLE + 2] = PHASE1_ID;
            lf_checker_rt::callee_cdecl!(
                C_SUBMIT, u32, voice, 0, 0, 1, block, triple,
                rd32(this.wrapping_add(THIS_X120)), slot
            );
        }
        // Phase 2: the global default voice id; block word 0 cleared first.
        frame[F_BLOCK] = 0;
        let slot2 = lf_checker_rt::callee_cdecl!(C_SLOT_ALLOC, u32,);
        let resolved2 = lf_checker_rt::callee_cdecl!(C_SLOT_RESOLVE, u32, slot2);
        let def = lf_checker_rt::global::<u32>(DEFAULT_VOICE).read();
        let ok2 =
            lf_checker_rt::callee_thiscall!(C_ISSUE, u32, this, def, block, slot2, resolved2, 0);
        if (ok2 as u8) == 0 {
            return lf_checker_rt::callee_cdecl!(C_RELEASE, u32, slot2);
        }
        frame[F_TRIPLE] = 0;
        frame[F_TRIPLE + 1] = 0xFFFF_FFFF;
        frame[F_TRIPLE + 2] = PHASE2_ID;
        lf_checker_rt::callee_cdecl!(
            C_SUBMIT, u32, def, 0, 0, 1, block, triple,
            rd32(this.wrapping_add(THIS_X120)), slot2
        )
    }
});
