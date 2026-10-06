// original: 0x0089c720 rage::audSpeechSound::vf9

/// Attaches the resolved speech voice to its slot parameters.
///
/// Resolves the speech entry as `TABLE2[bank*0x6f40+0x6f14] +
/// STRIDE2*speech_slot` (bank at `+0x40`, speech slot at `+0xb4`), runs the
/// pitch helper (callee 1, cdecl/1) on the SIGNED word at `+0x3c`, and looks
/// up the voice (callee 2, cdecl/2) from the helper's answer and the entry's
/// first word. A null voice sets the dead bit (bit 0 at `+0x39`) and returns
/// 0. Otherwise the voice's words at `+0x1a`/`+0x18`/`+0x10` are copied
/// through the convert helper (callee 3, cdecl/2) into the slot target at
/// `TABLE[bank*0x6f40+0x6f10]+STRIDE*slot` (`+0xe4`/`+0xe0`), and the flag at
/// target `+0xee` has bit 2 set or cleared by the SIGNED sign of the voice's
/// word at `+0x14` (the original uses `jl`). Returns the convert result.
/// The two stack words are never read (kept for the callee-pop balance).
///
/// Original: 0x0089c720 (thiscall, two unread stack words).
lf_checker_rt::export!(thiscall, rw_0089c720(this: u32, _a0: u32, _a1: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x39;
        const DEAD_BIT: u8 = 0x01;
        const PITCH_OFF: u32 = 0x3c;
        const BANK_OFF: u32 = 0x40;
        const SLOT_OFF: u32 = 0x48;
        const SPEECH_OFF: u32 = 0xb4;
        const NO_SLOT: u32 = 0xff;
        const TABLE_GLOB: u32 = 0x0115d988;
        const STRIDE_GLOB: u32 = 0x0115d964;
        const STRIDE2_GLOB: u32 = 0x0115d968;
        const ROW_STRIDE: u32 = 0x6f40;
        const VOICE_SLOT: u32 = 0x6f10;
        const SPEECH_SLOT: u32 = 0x6f14;
        const DST_PARAM: u32 = 0xe0;
        const DST_PITCH: u32 = 0xe4;
        const DST_FLAG: u32 = 0xee;
        const DST_BIT: u8 = 0x04;
        const PITCH: u32 = 1;
        const LOOKUP: u32 = 2;
        const CONVERT: u32 = 3;
        let table = lf_checker_rt::global::<u32>(TABLE_GLOB).read_unaligned();
        let bank = (this as *const u8).byte_add(BANK_OFF as usize).read() as u32;
        let speech = (this as *const u8).byte_add(SPEECH_OFF as usize).read() as u32;
        let stride2 = lf_checker_rt::global::<u32>(STRIDE2_GLOB).read_unaligned();
        let base2 = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
            .byte_add(SPEECH_SLOT as usize).read_unaligned();
        let entry = base2.wrapping_add(stride2.wrapping_mul(speech));
        let pitch = (this as *const i16).byte_add(PITCH_OFF as usize).read_unaligned() as i32 as u32;
        let h: u32 = lf_checker_rt::callee_cdecl!(PITCH, u32, pitch);
        let first = (entry as *const u32).read_unaligned();
        let voice: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, h, first);
        if voice == 0 {
            let f = (this as *mut u8).byte_add(FLAG_OFF as usize);
            f.write(f.read() | DEAD_BIT);
            return 0;
        }
        let slot = (this as *const u8).byte_add(SLOT_OFF as usize).read() as u32;
        let target = if slot == NO_SLOT {
            0
        } else {
            let stride = lf_checker_rt::global::<u32>(STRIDE_GLOB).read_unaligned();
            let base = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
                .byte_add(VOICE_SLOT as usize).read_unaligned();
            base.wrapping_add(stride.wrapping_mul(slot))
        };
        let w1a = (voice as *const u16).byte_add(0x1a).read_unaligned();
        (target as *mut u16).byte_add(DST_PITCH as usize).write_unaligned(w1a);
        let w18 = (voice as *const u16).byte_add(0x18).read_unaligned() as u32;
        let w10 = (voice as *const u32).byte_add(0x10).read_unaligned();
        let conv: u32 = lf_checker_rt::callee_cdecl!(CONVERT, u32, w10, w18);
        (target as *mut u32).byte_add(DST_PARAM as usize).write_unaligned(conv);
        let cell = (voice as *const u32).byte_add(0x14).read_unaligned();
        let f = (target as *mut u8).byte_add(DST_FLAG as usize);
        // SIGNED sign test (original `jl`): negative clears the bit.
        if (cell as i32) < 0 {
            f.write(f.read() & !DST_BIT);
        } else {
            f.write(f.read() | DST_BIT);
        }
        conv
    }
});
