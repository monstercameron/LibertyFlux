// original: 0x005D4470 network_array_object_is_idle

/// Determine whether the object's network-array slot is idle. The item points
/// to a network record at offset 0xF50, whose state object is at offset 0x22C.
/// Only state type 2 and a non-sentinel slot index reach the shared 80-byte
/// slot table. Flags 1, 2, 3, and 4 mean busy; every other low-three-bit value
/// is idle. The cdecl helper returns the boolean in AL.
lf_checker_rt::export!(cdecl, rw_005d4470(item: u32) -> u32 {
    const ITEM_RECORD_OFFSET: u32 = 0xF50;
    const STATE_OBJECT_OFFSET: u32 = 0x22C;
    const STATE_TYPE_OFFSET: u32 = 4;
    const SLOT_INDEX_OFFSET: u32 = 0x54;
    const SLOT_TABLE_VA: u32 = 0x019D5A10;
    const SLOT_STRIDE: u32 = 80;
    const FLAGS_OFFSET: u32 = 0x11;
    const EXPECTED_STATE_TYPE: u32 = 2;
    const NO_SLOT: u32 = u32::MAX;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let record = unsafe { read_u32(item.wrapping_add(ITEM_RECORD_OFFSET)) };
    if record == 0 {
        return 1;
    }
    let state = unsafe { read_u32(record.wrapping_add(STATE_OBJECT_OFFSET)) };
    if state == 0 {
        return 1;
    }
    if unsafe { read_u32(state.wrapping_add(STATE_TYPE_OFFSET)) } != EXPECTED_STATE_TYPE {
        return 1;
    }
    let slot_index = unsafe { read_u32(state.wrapping_add(SLOT_INDEX_OFFSET)) };
    if slot_index == NO_SLOT {
        return 1;
    }
    let slot = lf_checker_rt::relocated(SLOT_TABLE_VA)
        .wrapping_add(slot_index.wrapping_mul(SLOT_STRIDE));
    if slot == 0 {
        return 1;
    }
    let flags = unsafe { (slot.wrapping_add(FLAGS_OFFSET) as *const u8).read_unaligned() } & 7;
    u32::from(!matches!(flags, 1 | 2 | 3 | 4))
});
