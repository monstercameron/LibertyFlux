// original: 0x00C78100 phbound_register_entry (proposed)

/// Store the object pointer in its registry slot and return whether the slot
/// is non-null. For a non-null object with a nonzero 16-bit registration
/// count, call the slot helper and then the visitor with the embedded object,
/// callback address, and a pointer to the requested slot index. The registry
/// slot comes from the global table's flags, entry base, and byte stride.
lf_checker_rt::export!(cdecl, rw_phbound_register_entry(index: u32, object: u32) -> u32 {
    unsafe {
        const TABLE_GLOBAL: u32 = 0x016e_c7b8;
        const TABLE_ENTRIES: u32 = 0;
        const TABLE_FLAGS: u32 = 4;
        const TABLE_STRIDE: u32 = 0x0c;
        const ENTRY_OBJECT: u32 = 0;
        const OBJECT_REGISTRATION_COUNT: u32 = 0x14;
        const SLOT_HELPER: u32 = 1;
        const ENTRY_VISITOR: u32 = 2;
        const SLOT_CONTEXT: u32 = 0x30;
        const VISITOR_CALLBACK: u32 = 0x00c7_8e30;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read_volatile() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let table = unsafe { lf_checker_rt::global::<u32>(TABLE_GLOBAL).read_volatile() };
        let flags = unsafe { read_u32(table + TABLE_FLAGS) };
        let slot_is_flagged = unsafe { read_u8(flags.wrapping_add(index)) } & 0x80 != 0;
        let entries = unsafe { read_u32(table + TABLE_ENTRIES) };
        let stride = unsafe { read_u32(table + TABLE_STRIDE) };
        let slot = if slot_is_flagged {
            0
        } else {
            entries.wrapping_add(stride.wrapping_mul(index))
        };
        unsafe { write_u32(slot + ENTRY_OBJECT, object) };

        if object != 0 {
            let registration_count = unsafe { read_u16(object + OBJECT_REGISTRATION_COUNT) };
            if registration_count != 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    SLOT_HELPER,
                    u32,
                    slot.wrapping_add(SLOT_CONTEXT),
                    u32::from(registration_count),
                );
                let mut requested_index = index;
                let index_snapshot = core::ptr::addr_of_mut!(requested_index) as u32;
                let callback = lf_checker_rt::relocated(VISITOR_CALLBACK);
                let _ = lf_checker_rt::callee_thiscall!(
                    ENTRY_VISITOR,
                    u32,
                    object,
                    callback,
                    index_snapshot,
                );
            }
        }
        u32::from(object != 0)
    }
});
