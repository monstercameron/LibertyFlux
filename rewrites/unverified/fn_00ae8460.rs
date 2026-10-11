// original: 0x00AE8460 unregister_object_timer_slots

/// Removes the entries in a manager's active timer-slot range. The manager
/// has the range endpoints at `+0x2c` and `+0x30`, a primary bit index at
/// `+0x900`, a callback category at `+0x938`, and a callback flag at
/// `+0x8e8`. Each eight-byte slot begins with an object pointer. A non-null
/// object whose byte at `+0x24` lacks bit `0x20` is unlinked, has the primary
/// bit cleared from its words at `+0x54` and `+0x58`, and is passed to the
/// callback helper with the manager's category and the low bit of the high
/// flag nibble. The selected bit in the object's word at `+0x7c` is then
/// cleared using the sixth stack value, and the second bit index clears the
/// same two words at `+0x54` and `+0x58`. X86 shift counts use their low five
/// bits. The first stack argument slot is also overwritten with the manager's
/// primary bit index; this scratch write is part of the observed stack state.
///
/// Calling convention: cdecl with seven stack words; the first is a pointer
/// to the 32-bit manager layout and the seventh supplies the second bit index.
lf_checker_rt::export!(cdecl, rw_00ae8460(manager: u32, _arg2: u32, _arg3: u32, _arg4: u32, _arg5: u32, _arg6: u32, second_bit_index: u32) -> () {
    unsafe {
        const LIST_BEGIN: u32 = 0x2c;
        const LIST_END: u32 = 0x30;
        const PRIMARY_BIT_INDEX: u32 = 0x900;
        const CALLBACK_CATEGORY: u32 = 0x938;
        const CALLBACK_FLAGS: u32 = 0x8e8;
        const SLOT_OBJECT_FLAGS: u32 = 0x24;
        const OBJECT_LOW_FLAGS: u32 = 0x54;
        const OBJECT_LOW_FLAGS_MIRROR: u32 = 0x58;
        const OBJECT_CATEGORY_FLAGS: u32 = 0x7c;
        const SLOT_STRIDE: u32 = 8;
        const SKIP_CALLBACK: u8 = 0x20;
        const PRESERVE_HIGH_BYTE: u32 = 0xff00_0000;
        const CALLBACK_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let manager_address = manager;
        let mut incoming_manager_slot = manager;
        let range_begin = unsafe { read_u32(manager_address.wrapping_add(LIST_BEGIN)) };
        let range_end = unsafe { read_u32(manager_address.wrapping_add(LIST_END)) };
        let primary_index = unsafe { read_u32(manager_address.wrapping_add(PRIMARY_BIT_INDEX)) };
        let callback_category = unsafe { read_u32(manager_address.wrapping_add(CALLBACK_CATEGORY)) };
        let callback_flags = unsafe { read_u32(manager_address.wrapping_add(CALLBACK_FLAGS)) };

        // The native routine reuses the caller's first argument slot for this
        // category. Taking the address preserves that ABI-visible write.
        unsafe { core::ptr::addr_of_mut!(incoming_manager_slot).write_unaligned(primary_index) };

        let callback_flag = (callback_flags >> 28) & 1;
        let mut slot = range_begin;
        while slot != range_end {
            let object = unsafe { read_u32(slot) };
            if object != 0 {
                let object_flags = unsafe { read_u8(object.wrapping_add(SLOT_OBJECT_FLAGS)) };
                if object_flags & SKIP_CALLBACK == 0 {
                    unsafe { write_u32(slot, 0) };

                    let primary_mask = 1u32.wrapping_shl(primary_index);
                    let keep_primary = !primary_mask | PRESERVE_HIGH_BYTE;
                    let low_flags = unsafe { read_u32(object.wrapping_add(OBJECT_LOW_FLAGS)) };
                    unsafe { write_u32(object.wrapping_add(OBJECT_LOW_FLAGS), low_flags & keep_primary) };
                    let mirrored_flags = unsafe { read_u32(object.wrapping_add(OBJECT_LOW_FLAGS_MIRROR)) };
                    unsafe { write_u32(object.wrapping_add(OBJECT_LOW_FLAGS_MIRROR), mirrored_flags & keep_primary) };

                    let _ = lf_checker_rt::callee_cdecl!(
                        CALLBACK_CALLEE,
                        u32,
                        object,
                        callback_flag,
                        callback_category,
                        manager_address,
                        1,
                    );

                    let current_primary = unsafe { read_u32(manager_address.wrapping_add(PRIMARY_BIT_INDEX)) };
                    let primary_word_mask = 1u16.wrapping_shl(current_primary);
                    let category_flags = unsafe { read_u16(object.wrapping_add(OBJECT_CATEGORY_FLAGS)) };
                    unsafe { write_u16(object.wrapping_add(OBJECT_CATEGORY_FLAGS), category_flags & !primary_word_mask) };

                    let secondary_mask = 1u32.wrapping_shl(second_bit_index);
                    let keep_secondary = !secondary_mask | PRESERVE_HIGH_BYTE;
                    let low_flags = unsafe { read_u32(object.wrapping_add(OBJECT_LOW_FLAGS)) };
                    unsafe { write_u32(object.wrapping_add(OBJECT_LOW_FLAGS), low_flags & keep_secondary) };
                    let mirrored_flags = unsafe { read_u32(object.wrapping_add(OBJECT_LOW_FLAGS_MIRROR)) };
                    unsafe { write_u32(object.wrapping_add(OBJECT_LOW_FLAGS_MIRROR), mirrored_flags & keep_secondary) };
                }
            }
            slot = slot.wrapping_add(SLOT_STRIDE);
        }
    }
});
