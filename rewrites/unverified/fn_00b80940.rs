// original: 0x00B80940 ped_task_manager_clear_and_destroy_entries

/// Destroy each non-null object in the five primary slots, three list-head
/// slots, and six secondary slots, in that order, passing flag one to the
/// object's deleting method. Clear every slot even when it was already null.
/// The routine is void; EAX is whatever the last helper left behind.
lf_checker_rt::export!(thiscall, rw_b80940(this: u32) -> u32 {
    unsafe {
        const PRIMARY_COUNT: u32 = 5;
        const LIST_HEADS: u32 = 0x2c;
        const HEAD_COUNT: u32 = 3;
        const SECONDARY: u32 = 0x14;
        const SECONDARY_COUNT: u32 = 6;
        const DELETE_METHOD: u32 = 1;
        const DELETE_FLAG: u32 = 1;
        let destroy_and_clear = |slot: u32| {
            unsafe {
                let object = (slot as *const u32).read_unaligned();
                if object != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(DELETE_METHOD, u32, object, DELETE_FLAG);
                }
                (slot as *mut u32).write_unaligned(0);
            }
        };

        for index in 0..PRIMARY_COUNT {
            destroy_and_clear(this.wrapping_add(index * 4));
        }
        for index in 0..HEAD_COUNT {
            destroy_and_clear(this.wrapping_add(LIST_HEADS + index * 4));
        }
        for index in 0..SECONDARY_COUNT {
            destroy_and_clear(this.wrapping_add(SECONDARY + index * 4));
        }
        0
    }
});
