// original: 0x00AE8390 object_timer_service

/// Updates a positive signed number of timer-owned objects. Each list entry
/// points to an object with a category at `+0x938`, a timer mask at `+0x8e8`,
/// and two time fields at `+0x2c` and `+0x30`. The routine copies the current
/// global tick to both time fields, advancing a shared 16-bit counter between
/// the stores. At counter wrap it calls the reset helper and starts the count
/// at one. When the object's timer mask is enabled, it scans the global
/// eight-byte timer-entry table and calls the entry helper for each non-null
/// entry whose mask at `+8` contains `1 << category` (the shift count is
/// reduced to the low five bits, as on x86). Non-positive counts do no work.
/// The two time fields are read separately from the global tick around the
/// helper calls, and the upper half of the shared counter word is preserved.
///
/// Calling convention: cdecl with `objects` as a pointer to a 32-bit pointer
/// array and `count` interpreted as a signed 32-bit count. The object and
/// timer-entry layouts remain 32-bit because their pointers and fields are
/// part of the original data layout.
lf_checker_rt::export!(cdecl, rw_00ae8390(objects: u32, count: u32) -> () {
    unsafe {
        const CURRENT_TICK_VA: u32 = 0x0159_3bc8;
        const TIMER_COUNTER_VA: u32 = 0x011a_8908;
        const TIMER_ENTRIES_VA: u32 = 0x0159_8718;
        const TIMER_ENTRIES_END_VA: u32 = 0x0159_af1c;
        const OBJECT_TIME_FIRST: u32 = 0x2c;
        const OBJECT_TIME_SECOND: u32 = 0x30;
        const OBJECT_TIMER_MASK: u32 = 0x8e8;
        const OBJECT_CATEGORY: u32 = 0x938;
        const ENTRY_MASK: u32 = 8;
        const TIMER_ENABLED: u32 = 0x0080_0000;
        const RESET_CALLEE: u32 = 1;
        const ENTRY_CALLEE: u32 = 2;

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

        if count as i32 <= 0 {
            return;
        }

        let timer_counter = lf_checker_rt::relocated(TIMER_COUNTER_VA);
        for index in 0..count {
            let object = unsafe { read_u32(objects.wrapping_add(index.wrapping_mul(4))) };
            let first_tick = unsafe { read_u32(lf_checker_rt::relocated(CURRENT_TICK_VA)) };
            unsafe { write_u32(object.wrapping_add(OBJECT_TIME_FIRST), first_tick) };

            let old_counter = unsafe { read_u16(timer_counter) };
            if old_counter == u16::MAX {
                let _ = lf_checker_rt::callee_cdecl!(RESET_CALLEE, u32);
                unsafe { write_u16(timer_counter, 1) };
            } else {
                unsafe { write_u16(timer_counter, old_counter.wrapping_add(1)) };
            }

            let category = unsafe { read_u32(object.wrapping_add(OBJECT_CATEGORY)) };
            let timer_mask = unsafe { read_u32(object.wrapping_add(OBJECT_TIMER_MASK)) };
            if timer_mask & TIMER_ENABLED != 0 {
                let category_bit = 1u32.wrapping_shl(category);
                let mut entry_slot = lf_checker_rt::relocated(TIMER_ENTRIES_VA);
                let entries_end = unsafe {
                    read_u32(lf_checker_rt::relocated(TIMER_ENTRIES_END_VA))
                };
                while entry_slot < entries_end {
                    let entry = unsafe { read_u32(entry_slot) };
                    if entry != 0
                        && unsafe { read_u32(entry.wrapping_add(ENTRY_MASK)) } & category_bit != 0
                    {
                        let _ = lf_checker_rt::callee_cdecl!(
                            ENTRY_CALLEE,
                            u32,
                            entry,
                            category,
                            object,
                        );
                    }
                    entry_slot = entry_slot.wrapping_add(8);
                }
            }

            let second_tick = unsafe { read_u32(lf_checker_rt::relocated(CURRENT_TICK_VA)) };
            unsafe { write_u32(object.wrapping_add(OBJECT_TIME_SECOND), second_tick) };
        }
    }
});
