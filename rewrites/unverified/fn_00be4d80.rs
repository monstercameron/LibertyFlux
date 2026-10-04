// original: 0x00be4d80 task_effect_tune_and_reset_slot_b (proposed)

/// Retune one effect object, then reset a slot's handle exactly like its twin.
///
/// `obj` (second stack word; the first is unread) is retuned by the tuning
/// callee with the fixed float -4.0f passed by bits, and its flag word at
/// `obj + FLAG_OFF` (0x4) gains `FLAG_BIT` (0x400000). `slot` (third stack
/// word) is reset the same way as the neighbouring slot reset: its handle at
/// `+ HANDLE_OFF` (0x14) cleared and its flag at `+ FLAG2_OFF` (0x20) set to
/// one. Returns `slot` unchanged.
///
/// Original: 0x00be4d80 (cdecl, three stack words: unused, object, slot).
lf_checker_rt::export!(cdecl, rw_00be4d80(_unused: u32, obj: u32, slot: u32) -> u32 {
    unsafe {
        const TUNE_BITS: u32 = 0xc0800000; // -4.0f
        const FLAG_OFF: u32 = 0x04;
        const FLAG_BIT: u32 = 0x00400000;
        const HANDLE_OFF: u32 = 0x14;
        const FLAG2_OFF: u32 = 0x20;
        const TUNE: u32 = 1;
        lf_checker_rt::callee_thiscall!(TUNE, u32, obj, TUNE_BITS);
        let flags = (obj.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
        (obj.wrapping_add(FLAG_OFF) as *mut u32).write_unaligned(flags | FLAG_BIT);
        (slot.wrapping_add(HANDLE_OFF) as *mut u32).write_unaligned(0);
        (slot.wrapping_add(FLAG2_OFF) as *mut u8).write(1);
        slot
    }
});
