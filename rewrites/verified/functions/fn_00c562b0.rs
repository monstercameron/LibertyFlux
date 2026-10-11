// original: 0x00c562b0 release_event_reference_slots
/// Release up to seventeen reference slots on this object. For the sixteen consecutive dword slots at +0x18 through +0x54 and the final slot at +0x58, call the release helper with the old non-null value in ECX and the slot address as its argument, then clear the slot. The method has no defined return value.
lf_checker_rt::export!(thiscall, rw_00c562b0(this: u32) -> u32 {
unsafe {
    const FIRST_SLOT: u32 = 0x18;
    const SLOT_COUNT: u32 = 16;
    for index in 0..SLOT_COUNT {
        let slot = this.wrapping_add(FIRST_SLOT + index * 4);
        let previous = core::ptr::read_unaligned(slot as *const u32);
        if previous != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, previous, slot);
            core::ptr::write_unaligned(slot as *mut u32, 0);
        }
    }
    let final_slot = this.wrapping_add(0x58);
    let previous = core::ptr::read_unaligned(final_slot as *const u32);
    if previous != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, previous, final_slot);
        core::ptr::write_unaligned(final_slot as *mut u32, 0);
    }
}
0
});
