// original: 0x0097d5c0 audio_positional_voice_request (proposed)

/// Request a positional audio voice, or delegate to the default-voice update.
///
/// `this` is the audio manager object (dword at `+0x08` carried into the
/// request, dword at `+0x120` passed to the submit call). `key` selects a
/// voice bank through the bank lookup; `pos_obj` points at an object whose
/// three floats at `+0x10`/`+0x14`/`+0x18` are the request position; the
/// third stack word is never read.
///
/// Behaviour: three global gates must pass (init flag not 1, two epoch
/// words equal, mode not 0x12) or the function returns at once. The bank
/// lookup result's dword at `+0x16` is the voice id: 0 returns 0, and the
/// default-voice sentinel (global) delegates to the parameterless update
/// and returns its answer. Otherwise a request block is built in a frame
/// scratch area (block at frame `+0x38`, position copied to frame `+0x20`,
/// `this+0x08` stored at block `+0x20`, position address at block `+0x14`),
/// a slot is allocated and resolved, and the request is issued: success
/// submits it (eight-word submit call) and returns the submit answer,
/// failure releases the slot and returns the release answer.
///
/// Floats are only copied, never computed, so no arithmetic order applies.
/// The gate-1 exit path returns the caller's entry EAX, which a rewrite
/// cannot observe; the proof keeps gate 1 passing and notes this.
///
/// Original: 0x0097D5C0 (thiscall, three stack words, the third unread).
lf_checker_rt::export!(thiscall, rw_0097d5c0(this: u32, key: u32, pos_obj: u32, _unused: u32) -> u32 {
    unsafe {
        const GATE_INIT: u32 = 0x11F7060;
        const GATE_EPOCH_A: u32 = 0x12088B4;
        const GATE_EPOCH_B: u32 = 0xF1C040;
        const GATE_MODE: u32 = 0x1037720;
        const GATE_MODE_EXIT: u32 = 0x12;
        const BANK_OBJ: u32 = 0x115D9A0;
        const DEFAULT_VOICE: u32 = 0x12314A0;
        const VOICE_ID_OFF: u32 = 0x16;
        const THIS_TAG: u32 = 0x08;
        const THIS_X120: u32 = 0x120;
        const POS_X: u32 = 0x10;
        const POS_Y: u32 = 0x14;
        const POS_Z: u32 = 0x18;
        // Frame scratch mirrors the original's esp-relative layout so the
        // call-time snapshots observe the same geometry.
        const F_POS: usize = 0x20 / 4;
        const F_BLOCK: usize = 0x38 / 4;
        const F_SLOT_ALIAS: usize = 0x14 / 4;
        const BLOCK_POS_PTR: usize = 0x14 / 4;
        const BLOCK_TAG: usize = 0x20 / 4;
        const C_BANK: u32 = 1;
        const C_DEFAULT_UPDATE: u32 = 2;
        const C_BLOCK_INIT: u32 = 3;
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
        let bank = lf_checker_rt::callee_thiscall!(C_BANK, u32, lf_checker_rt::relocated(BANK_OBJ), key);
        if bank == 0 {
            return 0;
        }
        let voice = rd32(bank.wrapping_add(VOICE_ID_OFF));
        if voice == 0 {
            return 0;
        }
        if voice == lf_checker_rt::global::<u32>(DEFAULT_VOICE).read() {
            return lf_checker_rt::callee_thiscall!(C_DEFAULT_UPDATE, u32, this);
        }
        let mut frame = [0u32; 0x80];
        frame[F_POS] = rd32(pos_obj.wrapping_add(POS_X));
        frame[F_POS + 1] = rd32(pos_obj.wrapping_add(POS_Y));
        frame[F_POS + 2] = rd32(pos_obj.wrapping_add(POS_Z));
        let base = frame.as_mut_ptr() as u32;
        let block = base.wrapping_add((F_BLOCK * 4) as u32);
        lf_checker_rt::callee_thiscall!(C_BLOCK_INIT, u32, block);
        frame[F_BLOCK + BLOCK_TAG] = rd32(this.wrapping_add(THIS_TAG));
        frame[F_BLOCK + BLOCK_POS_PTR] = base.wrapping_add((F_POS * 4) as u32);
        let slot = lf_checker_rt::callee_cdecl!(C_SLOT_ALLOC, u32,);
        let resolved = lf_checker_rt::callee_cdecl!(C_SLOT_RESOLVE, u32, slot);
        let ok = lf_checker_rt::callee_thiscall!(C_ISSUE, u32, this, voice, block, slot, resolved, 0);
        if (ok as u8) == 0 {
            return lf_checker_rt::callee_cdecl!(C_RELEASE, u32, slot);
        }
        lf_checker_rt::callee_cdecl!(
            C_SUBMIT, u32, voice, 0, 0, 1, block,
            base.wrapping_add((F_SLOT_ALIAS * 4) as u32),
            rd32(this.wrapping_add(THIS_X120)), slot
        )
    }
});
