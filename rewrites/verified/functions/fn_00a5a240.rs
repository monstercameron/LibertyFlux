// original: 0x00A5A240 composite_bound_merge_item_masks

/// Merges the two mask words of every enabled item from four embedded tables
/// into the adjacent destination words. Each table has a signed count, a
/// parallel byte-enable array, and its own record stride; nonpositive counts
/// are skipped. The object pointer is in ECX. The function has no defined
/// return value; its observable output is the selected in-place OR updates.
lf_checker_rt::export!(thiscall, rw_00A5A240(this: u32) -> u32 {
    const TABLES: [(u32, u32, u32, u32); 4] = [
        (0x1950, 0x1960, 0x00d8, 0x00b0),
        (0x1954, 0x1980, 0x16d8, 0x00c0),
        (0x1958, 0x1981, 0x1798, 0x00e0),
        (0x195c, 0x1982, 0x1878, 0x0120),
    ];

    #[inline(always)]
    unsafe fn read_i32(address: u32) -> i32 {
        unsafe { (address as *const i32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn merge_table(base: u32, count_offset: u32, flag_offset: u32, dest_offset: u32, stride: u32) {
        let count = unsafe { read_i32(base.wrapping_add(count_offset)) };
        let mut index = 0i32;
        while index < count {
            let index_bytes = (index as u32).wrapping_mul(stride);
            let flags = base.wrapping_add(flag_offset).wrapping_add(index as u32);
            if unsafe { read_u8(flags) } != 0 {
                let destination = base.wrapping_add(dest_offset).wrapping_add(index_bytes);
                let source = destination.wrapping_sub(8);
                let low = unsafe { read_u32(destination) } | unsafe { read_u32(source) };
                let high = unsafe { read_u32(destination.wrapping_add(4)) }
                    | unsafe { read_u32(source.wrapping_add(4)) };
                unsafe {
                    write_u32(destination, low);
                    write_u32(destination.wrapping_add(4), high);
                }
            }
            index += 1;
        }
    }

    for (count_offset, flag_offset, dest_offset, stride) in TABLES {
        unsafe { merge_table(this, count_offset, flag_offset, dest_offset, stride) };
    }
    0
});
