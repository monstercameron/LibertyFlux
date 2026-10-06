// original: 0x0089cad0 rage::audSpeechSound::vf7

/// Acquires a speech slot for `this`, returning 1 on success, 0 on failure.
///
/// Runs the filter (callee 1, thiscall/3 on `this` with the three stack
/// words); a zero low byte fails. Otherwise allocates an 8-byte speech block
/// (callee 2, thiscall/2 on the sound pool with (8, bank)), failing on null,
/// clears it (`[0]=0`, `[4]=0xFFFF`), and derives the speech index from the
/// block's offset into the speech table (`TABLE2[bank*0x6f40+0x6f14]`,
/// divided UNSIGNED by `DIVISOR`), storing the low byte at `+0xb4`.
/// Registers the sound (callee 3, thiscall/3 on the registry with (`this`,
/// `a1`, `a2`)), failing on null, and derives the voice slot from that
/// answer's offset into the voice table (`TABLE[bank*0x6f40+0x6f10]`,
/// divided UNSIGNED by `STRIDE`), storing the low byte at `+0x48`. A slot of
/// 0xFF or a null `TABLE+STRIDE*slot` target fails; otherwise returns 1.
/// Only the low byte of EAX is defined (the original sets AL); the contract
/// compares AL.
///
/// Original: 0x0089cad0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_0089cad0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const BANK_OFF: u32 = 0x40;
        const SLOT_OFF: u32 = 0x48;
        const SPEECH_OFF: u32 = 0xb4;
        const NO_SLOT: u32 = 0xff;
        const TABLE_GLOB: u32 = 0x0115d988;
        const STRIDE_GLOB: u32 = 0x0115d964;
        const DIVISOR_GLOB: u32 = 0x0115d968;
        const SOUND_POOL: u32 = 0x0115d8a0;
        const REGISTRY: u32 = 0x0115dc18;
        const ROW_STRIDE: u32 = 0x6f40;
        const VOICE_SLOT: u32 = 0x6f10;
        const SPEECH_SLOT: u32 = 0x6f14;
        const FILTER: u32 = 1;
        const ALLOC: u32 = 2;
        const REGISTER: u32 = 3;
        let ok: u32 = lf_checker_rt::callee_thiscall!(FILTER, u32, this, a0, a1, a2);
        if ok & 0xff == 0 {
            return 0;
        }
        let bank = (this as *const u8).byte_add(BANK_OFF as usize).read() as u32;
        let blk: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, lf_checker_rt::relocated(SOUND_POOL), 8, bank);
        if blk == 0 {
            return 0;
        }
        (blk as *mut u32).write_unaligned(0);
        (blk as *mut u32).byte_add(4).write_unaligned(0xffff);
        let table = lf_checker_rt::global::<u32>(TABLE_GLOB).read_unaligned();
        let base2 = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
            .byte_add(SPEECH_SLOT as usize).read_unaligned();
        let divisor = lf_checker_rt::global::<u32>(DIVISOR_GLOB).read_unaligned();
        let speech = blk.wrapping_sub(base2).wrapping_div(divisor);
        (this as *mut u8).byte_add(SPEECH_OFF as usize).write(speech as u8);
        let reg: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, lf_checker_rt::relocated(REGISTRY), this, a1, a2);
        let slot = if reg == 0 {
            NO_SLOT
        } else {
            let stride = lf_checker_rt::global::<u32>(STRIDE_GLOB).read_unaligned();
            let base = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
                .byte_add(VOICE_SLOT as usize).read_unaligned();
            reg.wrapping_sub(base).wrapping_div(stride) & 0xff
        };
        (this as *mut u8).byte_add(SLOT_OFF as usize).write(slot as u8);
        if slot == NO_SLOT {
            return 0;
        }
        let stride = lf_checker_rt::global::<u32>(STRIDE_GLOB).read_unaligned();
        let base = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
            .byte_add(VOICE_SLOT as usize).read_unaligned();
        if base.wrapping_add(stride.wrapping_mul(slot)) == 0 {
            return 0;
        }
        1
    }
});
