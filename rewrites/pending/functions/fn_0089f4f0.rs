// original: 0x0089F4F0 aud_stream_slot_ptr
// ---------------------------------------------------------------------------
// 0x0089F4F0: look up the voice pointer for slot `index` of this sound.
// Returns null when the index is out of range (8 slots at +0xB0) or the slot
// is empty (0xFF). Otherwise returns the bank-table entry for this sound's
// bank byte (+0x40) plus stride * slot id. No calls; pure table lookup.
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089F4F0(this_ptr: u32, index: u32) -> u32 {
    const SLOT_COUNT: u32 = 8;
    const SLOT_EMPTY: u8 = 0xFF;
    const BANK_ROW: u32 = 0x6F40;
    const BANK_BASE_OFF: u32 = 0x6F14;
    if index >= SLOT_COUNT {
        return 0;
    }
    let slot = unsafe { *((this_ptr.wrapping_add(index).wrapping_add(0xB0)) as *const u8) };
    if slot == SLOT_EMPTY {
        return 0;
    }
    let bank_idx = unsafe { *((this_ptr.wrapping_add(0x40)) as *const u8) } as u32;
    let bank_table = unsafe { *global::<u32>(0x115D988) };
    let stride = unsafe { *global::<u32>(0x115D968) };
    let row = unsafe {
        *((bank_table
            .wrapping_add(bank_idx.wrapping_mul(BANK_ROW))
            .wrapping_add(BANK_BASE_OFF)) as *const u32)
    };
    row.wrapping_add(stride.wrapping_mul(slot as u32))
});
