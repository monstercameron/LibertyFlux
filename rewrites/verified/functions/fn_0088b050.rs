// original: 0x0088B050 audVoice_setSlotByte (proposed)

/// Set one voice-table slot byte from a scaled value, then notify the
/// voice through its own slot `+0x24`.
///
/// `this` points to the voice, `a0` is a voice handle relayed to the
/// notify entry, `a1` is the slot index and `a2` the raw value. Flag bit
/// 5 at `+0x3a` is always cleared. When `a1` is -1 the notify entry runs
/// with no store. When `a2` is zero the slot at `a1 + this + 0x48` is set
/// to 0xff. Otherwise the stored byte is the low byte of (`a2` minus the
/// table word for voice id `[this+0x40]`, reached through the voice-table
/// global) divided by the divisor global. The notify entry (callee 1)
/// then runs with the voice, `a0` and `a1`; its answer is the answer of
/// this function.
///
/// Original: 0x0088B050 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_0088B050(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const FLAG_BYTE: u32 = 0x3a;
        const KEEP_MASK: u8 = 0xdf;
        const VOICE_ID: u32 = 0x40;
        const SLOT_BASE: u32 = 0x48;
        const TABLE_GLOBAL: u32 = 0x0115_d988;
        const DIV_GLOBAL: u32 = 0x0115_d964;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BIAS: u32 = 0x6f10;
        const EMPTY_SLOT: u8 = 0xff;
        const NOTIFY: u32 = 1;

        let f = (this + FLAG_BYTE) as *mut u8;
        f.write(f.read() & KEEP_MASK);
        if a1 != 0xffff_ffff {
            let slot = (this.wrapping_add(a1).wrapping_add(SLOT_BASE)) as *mut u8;
            if a2 == 0 {
                slot.write(EMPTY_SLOT);
            } else {
                let vid = ((this + VOICE_ID) as *const u8).read() as u32;
                let tab = (lf_checker_rt::relocated(TABLE_GLOBAL)
                    as *const u32)
                    .read_unaligned();
                let w = ((tab
                    .wrapping_add(vid.wrapping_mul(ROW_STRIDE))
                    .wrapping_add(ROW_BIAS)) as *const u32)
                    .read_unaligned();
                let div = (lf_checker_rt::relocated(DIV_GLOBAL) as *const u32)
                    .read_unaligned();
                let q = a2.wrapping_sub(w) / div;
                slot.write(q as u8);
            }
        }
        lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, a0, a1)
    }
});
