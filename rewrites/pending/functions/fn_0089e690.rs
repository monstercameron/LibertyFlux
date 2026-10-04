// original: 0x0089e690 rage::audSequentialSound::vf9
/// `rage::audSequentialSound::vf9`: select the entry for the indexed slot and
/// record the index in the object when the entry exists. thiscall/2.
export!(thiscall, rw_0089e690(this: u32, _unused: u32, index: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6F40;
        const TABLE_OFF: u32 = 0x6F10;
        const ROW_IDX: u32 = 0x40;
        const SEL_BASE: u32 = 0x48;
        const SLOT: u32 = 0xD0;
        const STRIDE: u32 = 0x115D964;
        const TABLE: u32 = 0x115D988;
        const ABSENT: u32 = 0xFF;
        let sel = *(this.wrapping_add(index).wrapping_add(SEL_BASE) as *const u8) as u32;
        if sel == ABSENT {
            return ABSENT;
        }
        let row = *((this + ROW_IDX) as *const u8) as u32;
        let stride = *global::<u32>(STRIDE);
        let table = *global::<u32>(TABLE);
        let slot = table
            .wrapping_add(row.wrapping_mul(ROW_STRIDE))
            .wrapping_add(TABLE_OFF);
        let entry = (slot as *const u32).read_unaligned();
        let sum = entry.wrapping_add(sel.wrapping_mul(stride));
        if sum == 0 {
            return 0;
        }
        *((this + SLOT) as *mut u8) = (index & 0xFF) as u8;
        sum
    }
});
