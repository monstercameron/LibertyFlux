// original: 0x008a17b0 collapsing_stereo_slot_teardown
/// Slot teardown for a collapsing stereo sound with base tail call.
///
/// Stamps the vtable, then releases each of the 8 slot voices through its
/// vtable release slot (slot 5, stubbed via a planted table) and marks the
/// slot empty. Empty and null-voice slots are skipped (the original's
/// repeated re-checks all agree once the voice is known live). Ends by
/// tail-calling the base destructor (stubbed); its answer is the return.
export!(thiscall, rw_008a17b0(this: u32) -> u32 {
    unsafe {
        let o = this as *mut u8;
        *(o as *mut u32) = relocated(0xe7a894);
        let bank = *o.add(0x40);
        let mut i = 0usize;
        while i < 8 {
            let slot = *o.add(0x48 + i);
            i += 1;
            if slot == SLOT_EMPTY {
                continue;
            }
            let v = voice_ptr(bank, slot);
            if v == 0 {
                continue;
            }
            let vt = core::ptr::read_unaligned((v as *const u32));
            let tgt = core::ptr::read_unaligned(((vt.wrapping_add(0x14))) as *const u32);
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            release(v, 1);
            *o.add(0x48 + i - 1) = SLOT_EMPTY;
        }
        callee_thiscall!(2, u32, this)
    }
});
