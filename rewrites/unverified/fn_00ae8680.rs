// original: 0x00AE8680 process_timer_callbacks

/// Processes timer categories for each object in a signed-count array. The
/// manager uses flags at `+0x08`, category bits at `+0x0c`, a byte option at
/// `+0x26`, and a result byte at `+0x63`; each item has a vtable pointer at
/// `+0`, an enable byte at `+0x1a`, and a category index at `+0x938`.
/// It first asks the item vtable for its kind. Kind 21 may be filtered by a
/// manager vtable's elapsed-time result and the global threshold. Remaining
/// eligible items call the category handler; a nonzero AL result marks the
/// category active, and an enabled item also sets the manager result byte.
///
/// Calling convention: cdecl with a manager pointer, a 32-bit pointer array,
/// and a signed 32-bit item count. Object and vtable pointers stay 32-bit
/// because they are part of the original data layout.
lf_checker_rt::export!(cdecl, rw_00ae8680(manager: u32, items: u32, count: i32) -> () {
    unsafe {
        const MANAGER_FLAGS: u32 = 0x08;
        const MANAGER_CATEGORIES: u32 = 0x0c;
        const MANAGER_FILTER: u32 = 0x26;
        const MANAGER_RESULT: u32 = 0x63;
        const ITEM_ENABLED: u32 = 0x1a;
        const ITEM_CATEGORY: u32 = 0x938;
        const ITEM_KIND_SLOT: u32 = 0x24;
        const MANAGER_TIME_SLOT: u32 = 0x58;
        const TIMER_THRESHOLD_VA: u32 = 0x00fe_8b08;
        const SPECIAL_ITEM_KIND: u32 = 0x15;
        const STOP_FLAG: u32 = 0x4000_0000;
        const KIND_CALLEE: u32 = 1;
        const TIME_CALLEE: u32 = 2;
        const CATEGORY_CALLEE: u32 = 3;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write_unaligned(value) }
        }

        if read_u32(manager.wrapping_add(MANAGER_FLAGS)) & STOP_FLAG != 0 || count <= 0 {
            return;
        }

        let mut index = 0u32;
        while index < count as u32 {
            let item = read_u32(items.wrapping_add(index.wrapping_mul(4)));
            let category = read_u32(item.wrapping_add(ITEM_CATEGORY)) & 31;
            let category_bit = 1u32.wrapping_shl(category);

            if read_u32(manager.wrapping_add(MANAGER_FLAGS)) & category_bit != 0 {
                let item_vtable = read_u32(item);
                let item_kind_address = read_u32(item_vtable.wrapping_add(ITEM_KIND_SLOT));
                let item_kind: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(item_kind_address as usize);
                let kind = item_kind(item);

                let time_is_too_large = if kind == SPECIAL_ITEM_KIND
                    && read_u8(manager.wrapping_add(MANAGER_FILTER)) & 1 == 0
                {
                    let manager_vtable = read_u32(manager);
                    let elapsed_address = read_u32(manager_vtable.wrapping_add(MANAGER_TIME_SLOT));
                    let elapsed: extern "thiscall" fn(u32) -> f32 =
                        core::mem::transmute(elapsed_address as usize);
                    let elapsed_time = elapsed(manager);
                    let threshold = lf_checker_rt::global::<f32>(TIMER_THRESHOLD_VA).read_unaligned();
                    elapsed_time > threshold
                } else {
                    false
                };

                if !time_is_too_large
                    && read_u32(manager.wrapping_add(MANAGER_CATEGORIES)) & category_bit == 0
                {
                    let accepted = lf_checker_rt::callee_thiscall!(
                        CATEGORY_CALLEE,
                        u32,
                        manager,
                        item,
                    ) as u8;
                    if accepted != 0 {
                        let categories = read_u32(manager.wrapping_add(MANAGER_CATEGORIES));
                        write_u32(
                            manager.wrapping_add(MANAGER_CATEGORIES),
                            categories | category_bit,
                        );
                        if read_u8(item.wrapping_add(ITEM_ENABLED)) != 0 {
                            write_u8(manager.wrapping_add(MANAGER_RESULT), 0xff);
                        }
                    }
                }
            }

            index = index.wrapping_add(1);
        }
    }
});
