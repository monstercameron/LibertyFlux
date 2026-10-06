// original: 0x0089c330 rage::audSpeechSound::dtor_body (proposed)

/// Destructor body of `rage::audSpeechSound`: releases the voice, then chains.
///
/// Stamps the `audSpeechSound` virtual table (`VTABLE`, relocated like the
/// original's immediate store, whose constant carries a relocation entry),
/// then unless the skip bit (`SKIP_BIT` in the flags at `+0x39`) is set,
/// releases the voice selected by the bank/slot bytes (`+0x40`/`+0x48`):
/// a slot of 0xFF or a null
/// `TABLE[bank*0x6f40+0x6f10]+STRIDE*slot` target skips the release call
/// (callee 1, thiscall/1 on `this` with 0). Then, unless the speech slot at
/// `+0xb4` is 0xFF, the detach call (callee 2, thiscall/2 on the sound pool
/// with the bank and speech-slot bytes) runs and the speech slot is reset to
/// 0xFF. Control tail-jumps to the base destructor (callee 3, thiscall/0 on
/// `this`).
///
/// Original: 0x0089c330 (thiscall, no stack words; ends in a tail jump).
lf_checker_rt::export!(thiscall, rw_0089c330(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00e79e9c;
        const FLAG_OFF: u32 = 0x39;
        const SKIP_BIT: u8 = 0x40;
        const BANK_OFF: u32 = 0x40;
        const SLOT_OFF: u32 = 0x48;
        const SPEECH_OFF: u32 = 0xb4;
        const NO_SLOT: u32 = 0xff;
        const TABLE_GLOB: u32 = 0x0115d988;
        const STRIDE_GLOB: u32 = 0x0115d964;
        const SOUND_POOL: u32 = 0x0115d8a0;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_SLOT: u32 = 0x6f10;
        const RELEASE: u32 = 1;
        const DETACH: u32 = 2;
        const BASE_DTOR: u32 = 3;
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if rd8(this + FLAG_OFF) & SKIP_BIT == 0 {
            let slot = rd8(this + SLOT_OFF) as u32;
            if slot != NO_SLOT {
                let bank = rd8(this + BANK_OFF) as u32;
                let table = lf_checker_rt::global::<u32>(TABLE_GLOB).read_unaligned();
                let stride = lf_checker_rt::global::<u32>(STRIDE_GLOB).read_unaligned();
                let row = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
                    .byte_add(ROW_SLOT as usize).read_unaligned();
                let target = row.wrapping_add(stride.wrapping_mul(slot));
                if target != 0 {
                    lf_checker_rt::callee_thiscall!(RELEASE, u32, this, 0);
                }
            }
        }
        let speech = rd8(this + SPEECH_OFF) as u32;
        if speech != NO_SLOT {
            let bank = rd8(this + BANK_OFF) as u32;
            lf_checker_rt::callee_thiscall!(DETACH, u32, lf_checker_rt::relocated(SOUND_POOL), bank, speech);
            (this as *mut u8).byte_add(SPEECH_OFF as usize).write(NO_SLOT as u8);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
