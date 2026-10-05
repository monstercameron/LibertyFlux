// original: 0x0088F900 audSound_allocVoice (proposed)

/// Allocate a voice slot for this sound's voice id and record the slot
/// index at `+0x4`.
///
/// `this` points to the sound. The registry's allocator (callee 1) runs
/// with the registry global, kind 0xf0 and the voice id byte at `+0x40`;
/// when it returns null there is nothing more to do. Otherwise the slot
/// is committed (callee 2), and the recorded index is the low byte of the
/// allocated pointer minus the voice-table word for the voice id, divided
/// by the divisor global.
///
/// Original: 0x0088F900 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0088F900(this: u32) -> () {
    unsafe {
        const REGISTRY: u32 = 0x0115_d8a0;
        const KIND: u32 = 0xf0;
        const VOICE_ID: u32 = 0x40;
        const SLOT_INDEX: u32 = 0x04;
        const TABLE_GLOBAL: u32 = 0x0115_d988;
        const DIV_GLOBAL: u32 = 0x0115_d968;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BIAS: u32 = 0x6f14;
        const ALLOC: u32 = 1;
        const COMMIT: u32 = 2;

        let vid = ((this + VOICE_ID) as *const u8).read() as u32;
        let slot = lf_checker_rt::callee_thiscall!(
            ALLOC,
            u32,
            lf_checker_rt::relocated(REGISTRY),
            KIND,
            vid
        );
        if slot == 0 {
            return;
        }
        lf_checker_rt::callee_thiscall!(COMMIT, u32, slot);
        let tab =
            (lf_checker_rt::relocated(TABLE_GLOBAL) as *const u32).read_unaligned();
        let w = ((tab
            .wrapping_add(vid.wrapping_mul(ROW_STRIDE))
            .wrapping_add(ROW_BIAS)) as *const u32)
            .read_unaligned();
        let div =
            (lf_checker_rt::relocated(DIV_GLOBAL) as *const u32).read_unaligned();
        let q = slot.wrapping_sub(w) / div;
        ((this + SLOT_INDEX) as *mut u8).write(q as u8);
    }
});
