// original: 0x00C6F7D0 anim_filter_table_find

/// Scan the animation-filter pointer table for an object whose first two dwords equal the two arguments in order. The table count is an unsigned 16-bit global. Return the matching object pointer or zero; entries are assumed non-null by the original.
lf_checker_rt::export!(cdecl, rw_0x00c6f7d0(first_key: u32, second_key: u32) -> u32 {
    const FILTER_COUNT_GLOBAL: u32 = 0x016D_D658;
    const FILTER_TABLE_GLOBAL: u32 = 0x016D_D654;

    unsafe {
        let count = lf_checker_rt::global::<u16>(FILTER_COUNT_GLOBAL).read_unaligned() as u32;
        let entries = lf_checker_rt::global::<u32>(FILTER_TABLE_GLOBAL).read_unaligned() as *const u8;
        let mut index = 0u32;
        while index < count {
            let object = core::ptr::read_unaligned(entries.wrapping_add(index as usize * 4) as *const u32);
            let object_bytes = object as *const u8;
            let first = core::ptr::read_unaligned(object_bytes as *const u32);
            let second = core::ptr::read_unaligned(object_bytes.add(4) as *const u32);
            if first == first_key && second == second_key { return object; }
            index += 1;
        }
        0
    }
});
