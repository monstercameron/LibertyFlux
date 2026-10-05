// original: 0x00aa0850 stream_slot_adopt (proposed)

/// Adopt an orphan slot: record its owner generation, notify, and mark it.
///
/// `slot` points to a record with an owner word at `+0x70`. When the owner
/// is not `-1` the slot is already adopted and the owner word's own address
/// is returned unchanged. Otherwise `gen` (the second stack word; the third
/// is unused) is stored as the owner, the change hook is called with the
/// owner address, the slot is marked live through the neighbouring marker
/// routine, and the marker's answer is the result.
///
/// Original: 0x00aa0850 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00aa0850(this: u32, slot: u32, gen: u32, _unused: u32) -> u32 {
    unsafe {
        const OWNER_OFF: u32 = 0x70;
        const ORPHAN: u32 = 0xffff_ffff;
        const HOOK: u32 = 1;
        const MARK: u32 = 2;
        let owner = (slot + OWNER_OFF) as *mut u32;
        if owner.read_unaligned() != ORPHAN {
            return slot + OWNER_OFF;
        }
        owner.write_unaligned(gen);
        lf_checker_rt::callee_thiscall!(HOOK, u32, gen, slot + OWNER_OFF);
        lf_checker_rt::callee_thiscall!(MARK, u32, this, slot)
    }
});
