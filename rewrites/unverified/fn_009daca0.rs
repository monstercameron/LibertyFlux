// original: 0x009daca0 streaming_schedule_primary_entries
/// Walks the primary streaming work list using its item base, flag-byte base,
/// signed count, and stride. Entries with the high flag bit set are skipped.
/// For an item that resolves successfully, a nonzero filter byte schedules it
/// directly; otherwise the pending and status helpers decide whether it is
/// scheduled. This cdecl notification routine has no defined return value.
lf_checker_rt::export!(cdecl, rw_009daca0(filter_byte: u32) -> u32 {
    const LIST_POINTER_GLOBAL: u32 = 0x016e_c774;
    const CATEGORY_GLOBAL: u32 = 0x0104_b814;
    const ITEM_FLAGS: u8 = 0x80;
    const BLOCKING_STATUS: u8 = 0xc6;

    let list_address = unsafe {
        lf_checker_rt::global::<u32>(LIST_POINTER_GLOBAL).read_unaligned()
    };
    let list = list_address as *const u32;
    let item_base = unsafe { core::ptr::read_unaligned(list) };
    let flag_base = unsafe { core::ptr::read_unaligned(list.add(1)) };
    let item_count = unsafe { core::ptr::read_unaligned(list.add(2)) as i32 };
    let item_stride = unsafe { core::ptr::read_unaligned(list.add(3)) };
    let category = unsafe {
        lf_checker_rt::global::<u32>(CATEGORY_GLOBAL).read_unaligned()
    };
    let filter_byte = filter_byte as u8;
    let mut index = 0u32;

    while (index as i32) < item_count {
        let flags = unsafe {
            core::ptr::read_unaligned(flag_base.wrapping_add(index) as *const u8)
        };
        let item_address = item_base.wrapping_add(item_stride.wrapping_mul(index));
        if flags & ITEM_FLAGS == 0 && item_address != 0 {
            let item = lf_checker_rt::callee_cdecl!(1, u32, index);
            if item != 0 {
                if filter_byte != 0 {
                    let _scheduled = lf_checker_rt::callee_cdecl!(4, u32, index);
                } else {
                    let pending = lf_checker_rt::callee_cdecl!(2, u32, index);
                    if pending == 0 {
                        let status = lf_checker_rt::callee_cdecl!(3, u32, index, category) as u8;
                        if status & BLOCKING_STATUS == 0 {
                            let _scheduled = lf_checker_rt::callee_cdecl!(4, u32, index);
                        }
                    }
                }
            }
        }
        index = index.wrapping_add(1);
    }
    0
});
