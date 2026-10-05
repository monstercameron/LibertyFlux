// original: 0x00988BC0 audCutscene_teardown_slots (proposed)

/// Cutscene slot teardown: releases one or two slots, then drops the shared
/// voice when both slots are empty.
///
/// `flag` nonzero selects two rounds, else one. Rounds alternate the slot
/// index starting from `(byte at +SEL_OFF + 1) % 2` (the original's signed
/// modulo over a positive value is plain parity). A round whose slot word at
/// `+SLOT0_OFF + 4 * index` is null only clears the slot's byte at
/// `+SLOT_BYTE_OFF + 0x40 * index`. Otherwise the sequence value from the
/// global `SEQ` is stored at `+SEQ_OFF`, then an accumulator is resolved: the
/// slot entry's byte at `+KIND_OFF` of 0xff selects null (the following store
/// then faults, which the checker compares as fault parity), else the global
/// `WEIGHT_BASE` times that byte plus the table word at `TABLE_BASE` indexed
/// by the entry's byte at `+BAND_OFF` times `BAND_STRIDE`. The constant
/// `ACC_SET` is stored at `+ACC_OFF` of the accumulator, the entry is
/// released with argument 0 (callee id 1), and the slot word is cleared.
/// Afterwards, when the word at `+VOICE_OFF` is nonzero while both slot words
/// are zero, the voice is dropped (callee id 2) and its word cleared.
/// No meaningful return value. (The original keeps its round counter in its
/// incoming argument slot; the contract switches that comparison off and the
/// iteration count is observed through the release calls and slot writes.)
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00988BC0(this: u32, flag: u32) -> u32 {
    const SEL_OFF: u32 = 0xa0;
    const SLOT0_OFF: u32 = 8;
    const SLOT_BYTE_OFF: u32 = 0x14;
    const SLOT_BYTE_STRIDE: u32 = 0x40;
    const SEQ: u32 = 0x11735b4;
    const SEQ_OFF: u32 = 0x98;
    const KIND_OFF: u32 = 4;
    const KIND_NONE: u32 = 0xff;
    const BAND_OFF: u32 = 0x40;
    const BAND_STRIDE: u32 = 0x6f40;
    const WEIGHT_BASE: u32 = 0x115d968;
    const TABLE_BASE: u32 = 0x115d988;
    const TABLE_BIAS: u32 = 0x6f14;
    const ACC_OFF: u32 = 0xe0;
    const ACC_SET: u32 = 0x1f4;
    const VOICE_OFF: u32 = 0x10;
    const RELEASE: u32 = 1;
    const DROP_VOICE: u32 = 2;
    unsafe {
        let rounds = ((flag as u8) != 0) as u32 + 1;
        let sel = ((this + SEL_OFF) as *const u8).read() as u32;
        let mut idx = (sel + 1) % 2;
        let mut done = 0u32;
        while (done as i32) < (rounds as i32) {
            let slot_addr = this + SLOT0_OFF + idx * 4;
            let entry = (slot_addr as *const u32).read_unaligned();
            if entry != 0 {
                let seq = *lf_checker_rt::global::<u32>(SEQ);
                ((this + SEQ_OFF) as *mut u32).write_unaligned(seq);
                let kind = ((entry + KIND_OFF) as *const u8).read() as u32;
                let acc = if kind == KIND_NONE {
                    0
                } else {
                    let band = ((entry + BAND_OFF) as *const u8).read() as u32;
                    let w = *lf_checker_rt::global::<u32>(WEIGHT_BASE);
                    let t = *lf_checker_rt::global::<u32>(TABLE_BASE);
                    let word = (band
                        .wrapping_mul(BAND_STRIDE)
                        .wrapping_add(t)
                        .wrapping_add(TABLE_BIAS)
                        as *const u32)
                        .read_unaligned();
                    w.wrapping_mul(kind).wrapping_add(word)
                };
                ((acc + ACC_OFF) as *mut u32).write_unaligned(ACC_SET);
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(RELEASE, u32, entry, 0);
                (slot_addr as *mut u32).write_unaligned(0);
            }
            let baddr = this
                .wrapping_add(SLOT_BYTE_OFF)
                .wrapping_add(idx.wrapping_mul(SLOT_BYTE_STRIDE));
            (baddr as *mut u8).write(0);
            idx = (idx + 1) % 2;
            done += 1;
        }
        let voice = ((this + VOICE_OFF) as *const u32).read_unaligned();
        let s0 = ((this + SLOT0_OFF) as *const u32).read_unaligned();
        let s1 = ((this + SLOT0_OFF + 4) as *const u32).read_unaligned();
        if voice != 0 && s0 == 0 && s1 == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(DROP_VOICE, u32, voice);
            ((this + VOICE_OFF) as *mut u32).write_unaligned(0);
        }
    }
    0
});
