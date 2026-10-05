// original: 0x0097d6f0 audio_event_voice_select (proposed)

/// Select the voice for an audio event id and issue the request.
///
/// `this` is the audio manager object: dword at `+0x0C` must be zero (else
/// return at once), dword at `+0x70` receives the event id, byte at `+0x12A`
/// records whether the id is one of ten known event ids, dword at `+0x08`
/// is carried into the request block, dword at `+0x120` is the bank base
/// (zero skips the bank lookup). `key` is passed to the bank lookup;
/// `event` is the classified id.
///
/// Behaviour: the three global gates must pass or the function returns at
/// once. After storing and classifying the event, a locator call runs, then
/// the voice id resolves in stages: the bank entry word at `+0x0A` wins if
/// nonzero, else a fallback chain (object, its word at `+0x10`, that word's
/// `+0x16`) is tried while the id still equals the sentinel, else the
/// global fallback id is used. A request block is built in frame scratch
/// (block at frame `+0x14`, bank base at block `+0x0C`, tag at `+0x20`),
/// a slot is allocated and resolved, and the six-word issue call runs with
/// the voice id, `this+0x0C`, the block and the slot: success submits
/// (eight-word submit call, triple `0,-1,0x4C` beside the block) and
/// returns the submit answer, failure releases the slot and returns the
/// release answer.
///
/// No floating point is used. The gate-1 exit returns the caller's entry
/// EAX, which a rewrite cannot observe; the proof keeps gate 1 passing
/// and notes this.
///
/// Original: 0x0097D6F0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0097d6f0(this: u32, key: u32, event: u32) -> u32 {
    unsafe {
        const GATE_INIT: u32 = 0x11F7060;
        const GATE_EPOCH_A: u32 = 0x12088B4;
        const GATE_EPOCH_B: u32 = 0xF1C040;
        const GATE_MODE: u32 = 0x1037720;
        const GATE_MODE_EXIT: u32 = 0x12;
        const BANK_OBJ: u32 = 0x115D9A0;
        const SENTINEL: u32 = 0x1231310;
        const FALLBACK: u32 = 0x123130C;
        const THIS_GUARD: u32 = 0x0C;
        const THIS_EVENT: u32 = 0x70;
        const THIS_KNOWN: u32 = 0x12A;
        const THIS_LOCATOR: u32 = 0x60;
        const THIS_TAG: u32 = 0x08;
        const THIS_X120: u32 = 0x120;
        const BANK_BIAS: u32 = 0x780;
        const CHAIN_BIAS: u32 = 0x2B0;
        const ENTRY_ID_OFF: u32 = 0x0A;
        const SUBMIT_ID: u32 = 0x4C;
        const F_TRIPLE: usize = 0x08 / 4;
        const F_BLOCK: usize = 0x14 / 4;
        const F_LEA_A: u32 = 0x08;
        const BLOCK_BANK: usize = 0x0C / 4;
        const BLOCK_TAG: usize = 0x20 / 4;
        const C_LOCATOR: u32 = 1;
        const C_BLOCK_INIT: u32 = 2;
        const C_BANK: u32 = 3;
        const C_CHAIN: u32 = 4;
        const C_SLOT_ALLOC: u32 = 5;
        const C_SLOT_RESOLVE: u32 = 6;
        const C_ISSUE: u32 = 7;
        const C_SUBMIT: u32 = 8;
        const C_RELEASE: u32 = 9;

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
        if rd32(this.wrapping_add(THIS_GUARD)) != 0 {
            return epoch;
        }
        (this.wrapping_add(THIS_EVENT) as *mut u32).write_unaligned(event);
        let known = matches!(
            event,
            0x1A2 | 0x1A3 | 0x1A4 | 0x1A5 | 0x38B0 | 0x1A7 | 0x1A8 | 0x1A9 | 0x4B0 | 0x39B0
        );
        (this.wrapping_add(THIS_KNOWN) as *mut u8).write(if known { 1 } else { 0 });
        lf_checker_rt::callee_thiscall!(
            C_LOCATOR, u32, this, this.wrapping_add(THIS_LOCATOR)
        );
        let sentinel = lf_checker_rt::global::<u32>(SENTINEL).read();
        let mut voice = sentinel;
        let mut frame = [0u32; 0x80];
        let base = frame.as_mut_ptr() as u32;
        let block = base.wrapping_add((F_BLOCK * 4) as u32);
        lf_checker_rt::callee_thiscall!(C_BLOCK_INIT, u32, block);
        frame[F_BLOCK + BLOCK_TAG] = rd32(this.wrapping_add(THIS_TAG));
        let bank = rd32(this.wrapping_add(THIS_X120));
        if bank != 0 {
            frame[F_BLOCK + BLOCK_BANK] = bank.wrapping_add(BANK_BIAS);
            let found =
                lf_checker_rt::callee_thiscall!(C_BANK, u32, lf_checker_rt::relocated(BANK_OBJ), key);
            if found == 0 {
                return 0;
            }
            let id = rd32(found.wrapping_add(ENTRY_ID_OFF));
            if id != 0 {
                voice = id;
            }
            if voice == sentinel {
                let chained = lf_checker_rt::callee_thiscall!(
                    C_CHAIN, u32, bank.wrapping_add(CHAIN_BIAS)
                );
                if chained != 0 {
                    let mid = rd32(chained.wrapping_add(0x10));
                    if mid != 0 {
                        let cid = rd32(mid.wrapping_add(0x16));
                        if cid != 0 {
                            voice = cid;
                        }
                    }
                }
                if voice == sentinel {
                    voice = lf_checker_rt::global::<u32>(FALLBACK).read();
                }
            }
        } else {
            voice = lf_checker_rt::global::<u32>(FALLBACK).read();
        }
        let slot = lf_checker_rt::callee_cdecl!(C_SLOT_ALLOC, u32,);
        let resolved = lf_checker_rt::callee_cdecl!(C_SLOT_RESOLVE, u32, slot);
        let lea_b = base.wrapping_add((F_BLOCK * 4) as u32);
        let ok = lf_checker_rt::callee_thiscall!(
            C_ISSUE, u32, this, voice, this.wrapping_add(THIS_GUARD), lea_b, slot, resolved, 0
        );
        if (ok as u8) == 0 {
            return lf_checker_rt::callee_cdecl!(C_RELEASE, u32, slot);
        }
        frame[F_TRIPLE] = 0;
        frame[F_TRIPLE + 1] = 0xFFFF_FFFF;
        frame[F_TRIPLE + 2] = SUBMIT_ID;
        lf_checker_rt::callee_cdecl!(
            C_SUBMIT, u32, voice, 0, 1, 1, lea_b, base.wrapping_add(F_LEA_A),
            rd32(this.wrapping_add(THIS_X120)), slot
        )
    }
});
