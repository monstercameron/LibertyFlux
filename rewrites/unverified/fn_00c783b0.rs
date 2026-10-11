// original: 0x00C783B0 phbound_copy_entry (proposed)

/// Set the caller's result byte to zero, copy the context pointer at
/// `object+8` into the indexed registry entry, and ask the registry visitor
/// to process that context and the callback data. Valid entries are selected
/// through the global table's byte flags, base pointer, and four-byte stride.
/// The function returns true in AL.
lf_checker_rt::export!(cdecl, rw_phbound_copy_entry(index: u32, object: u32, callback_data: u32, result_slot: u32) -> u32 {
    unsafe {
        const TABLE_GLOBAL: u32 = 0x016e_c7b8;
        const TABLE_ENTRIES: u32 = 0;
        const TABLE_FLAGS: u32 = 4;
        const TABLE_STRIDE: u32 = 0x0c;
        const OBJECT_CONTEXT: u32 = 8;
        const VISIT_ENTRY: u32 = 1;
        const VISITOR_CALLBACK: u32 = 0x00c7_8400;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read_volatile() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write_volatile(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        unsafe { write_u8(result_slot, 0) };
        let table = unsafe { lf_checker_rt::global::<u32>(TABLE_GLOBAL).read_volatile() };
        let flags = unsafe { read_u32(table + TABLE_FLAGS) };
        if unsafe { read_u8(flags.wrapping_add(index)) } & 0x80 != 0 {
            // The original selects a null destination for a flagged entry;
            // this invalid-entry fault path is excluded by the contract.
            return 1;
        }

        let stride = unsafe { read_u32(table + TABLE_STRIDE) };
        let entries = unsafe { read_u32(table + TABLE_ENTRIES) };
        let destination = entries.wrapping_add(stride.wrapping_mul(index));
        let context = unsafe { read_u32(object + OBJECT_CONTEXT) };
        unsafe { write_u32(destination, context) };
        let visitor = lf_checker_rt::relocated(VISITOR_CALLBACK);
        let _ = lf_checker_rt::callee_thiscall!(VISIT_ENTRY, u32, context, visitor, callback_data);
        1
    }
});
