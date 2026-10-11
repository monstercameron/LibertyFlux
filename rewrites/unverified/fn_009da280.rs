// original: 0x009da280 streaming_apply_entry_flags_and_reset_active_marker
/// Walks the caller-owned record array using its 16-bit count and 0x30-byte
/// stride. For each non-null entry, it copies the owner's 0xC00 flag bits into
/// the entry, resolves the associated object, and clears that object's active
/// marker unless its four-bit type is 4 or 5. Eligible objects receive a
/// thiscall update and a cdecl notification. The cdecl routine has no defined
/// return value.
lf_checker_rt::export!(cdecl, rw_009da280(owner: u32) -> u32 {
    const ENTRY_ARRAY_OFFSET: u32 = 0x150;
    const ENTRY_COUNT_OFFSET: u32 = 0x154;
    const OWNER_FLAGS_OFFSET: u32 = 0x140;
    const ENTRY_STRIDE: u32 = 0x30;
    const ENTRY_FLAGS_OFFSET: u32 = 0x20;
    const OBJECT_TYPE_OFFSET: u32 = 0x28;
    const OBJECT_ACTIVE_OFFSET: u32 = 0x4c;
    const FLAG_MASK: u32 = 0x0c00;
    const FORMAT_STRING_VA: u32 = 0x00e9_6657;

    let owner_ptr = owner as *mut u8;
    let entry_base = unsafe {
        core::ptr::read_unaligned(owner_ptr.add(ENTRY_ARRAY_OFFSET as usize) as *const u32)
    };
    let count = unsafe {
        core::ptr::read_unaligned(owner_ptr.add(ENTRY_COUNT_OFFSET as usize) as *const u16) as u32
    };
    let owner_flags = unsafe {
        core::ptr::read_unaligned(owner_ptr.add(OWNER_FLAGS_OFFSET as usize) as *const u32)
    };
    let format = lf_checker_rt::relocated(FORMAT_STRING_VA);
    let mut index = 0u32;

    while (index as i32) < count as i32 {
        let entry_address = entry_base.wrapping_add(ENTRY_STRIDE.wrapping_mul(index));
        if entry_address != 0 {
            let entry = entry_address as *mut u8;
            let entry_flags = unsafe {
                core::ptr::read_unaligned(entry.add(ENTRY_FLAGS_OFFSET as usize) as *const u32)
            };
            let updated_flags = (owner_flags & !FLAG_MASK) | (entry_flags & FLAG_MASK);
            unsafe {
                core::ptr::write_unaligned(
                    entry.add(ENTRY_FLAGS_OFFSET as usize) as *mut u32,
                    updated_flags,
                );
            }

            let object = lf_checker_rt::callee_cdecl!(1, u32, entry_address, format);
            if object != 0 {
                let object_ptr = object as *mut u8;
                let object_flags = unsafe {
                    core::ptr::read_unaligned(
                        object_ptr.add(OBJECT_TYPE_OFFSET as usize) as *const u32,
                    )
                };
                let object_type = (object_flags >> 6) & 0x0f;
                if object_type != 4 && object_type != 5 {
                    unsafe {
                        core::ptr::write_unaligned(
                            object_ptr.add(OBJECT_ACTIVE_OFFSET as usize) as *mut u32,
                            0,
                        );
                    }
                    let _updated = lf_checker_rt::callee_thiscall!(
                        2,
                        u32,
                        object,
                        owner,
                        0,
                        u32::MAX
                    );
                    let _notified = lf_checker_rt::callee_cdecl!(3, u32, object, 0);
                }
            }
        }
        index = index.wrapping_add(1);
    }
    0
});
