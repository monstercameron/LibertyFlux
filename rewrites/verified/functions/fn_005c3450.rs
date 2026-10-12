// original: 0x005C3450 ped_factory_slot_lookup_by_index

/// Returns the object pointer at `manager.items + index * stride`, unless the
/// entry's flag byte has bit 7 set. The manager address is relocated through
/// the checker runtime; the table and bitmap pointers are fields of its
/// descriptor. The original flagged-entry path dereferences null and faults,
/// so the proof contract restricts inputs to clear flag bytes.
lf_checker_rt::export!(cdecl, rw_005c3450(index: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x015f8ba4;
        const ITEMS_POINTER: u32 = 0x00;
        const FLAGS_POINTER: u32 = 0x04;
        const ENTRY_STRIDE: u32 = 0x0c;
        const FLAGGED_MASK: u8 = 0x80;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned();
        let flags = read_word(manager + FLAGS_POINTER);
        if read_byte(flags.wrapping_add(index)) & FLAGGED_MASK != 0 {
            return 0;
        }
        let items = read_word(manager + ITEMS_POINTER);
        let stride = read_word(manager + ENTRY_STRIDE);
        read_word(items.wrapping_add(stride.wrapping_mul(index)))
    }
});
