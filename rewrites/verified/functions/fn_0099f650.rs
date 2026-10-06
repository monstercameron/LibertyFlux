// original: 0x0099F650 audio_megaphone_slot_present (proposed)

/// Report whether the entity's voice id occupies a megaphone slot.
///
/// The readiness gate (callee 1, thiscall/0) must answer nonzero in its low byte, else the shared tail returns the answer with that byte cleared.
/// result is 0. The four slot ids are initialised once: when the low bit of
/// the global flag is clear it is set and each id is fetched by the lookup
/// (callee 2, cdecl/2 of a relocated name address and zero) into the four
/// global slots. The voice id at `this`+0xA8 is then compared against the
/// four slots in order; a match returns 1. A miss returns 0: the loop
/// counter 0x10 fits in the low byte the shared tail clears. Thiscall with
/// no stack words.
lf_checker_rt::export!(thiscall, rw_0099F650(this: u32) -> u32 {
    unsafe {
        const VOICE: u32 = 0xA8;
        const FLAG: u32 = 0x012845B0;
        const SLOTS: u32 = 0x012845A0;
        const NAMES: [u32; 4] = [0x00E91208, 0x00E91220, 0x00E9123C, 0x00E91248];
        const READY_CALLEE: u32 = 1;
        const LOOKUP_CALLEE: u32 = 2;
        const MISS_RESULT: u32 = 0;
        let ready = lf_checker_rt::callee_thiscall!(READY_CALLEE, u32, this);
        if (ready as u8) == 0 {
            // Shared tail: only the low byte is cleared.
            return ready & 0xFFFF_FF00;
        }
        let flagp = lf_checker_rt::global::<u32>(FLAG) as *mut u32;
        if (flagp.read_unaligned() & 1) == 0 {
            flagp.write_unaligned(flagp.read_unaligned() | 1);
            let mut slot = lf_checker_rt::global::<u32>(SLOTS) as *mut u32;
            for i in 0..4 {
                let id = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32,
                                                      lf_checker_rt::relocated(NAMES[i]), 0);
                slot.write_unaligned(id);
                slot = slot.add(1);
            }
        }
        let voice = ((this.wrapping_add(VOICE)) as *const u32).read_unaligned();
        let mut slot = lf_checker_rt::global::<u32>(SLOTS) as *const u32;
        for _ in 0..4 {
            if slot.read_unaligned() == voice {
                return 1;
            }
            slot = slot.add(1);
        }
        MISS_RESULT
    }
});
