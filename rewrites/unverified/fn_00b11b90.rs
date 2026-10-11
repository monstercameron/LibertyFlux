// original: 0x00B11B90 advance_entity_value_by_range_table

/// Scan the range table's signed-positive item count. Each item has an
/// unsigned lower bound in the first array, an unsigned exclusive upper bound
/// at +0x200, and an unsigned delta at +0x400. If the value at `this + 0x18`
/// lies in a range, add its delta with 32-bit wrapping and store it back.
/// Values outside every range are left unchanged. This is a thiscall method
/// with the table pointer on the stack.
lf_checker_rt::export!(thiscall, rw_00b11b90(this: u32, table: u32) -> () {
    const VALUE: u32 = 0x18;
    const UPPER_BOUNDS: u32 = 0x200;
    const DELTAS: u32 = 0x400;
    const COUNT: u32 = 0x600;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let item_count = read_u32(table.wrapping_add(COUNT)) as i32;
        if item_count <= 0 {
            return;
        }
        let value_address = this.wrapping_add(VALUE);
        let value = read_u32(value_address);
        for item in 0..item_count as u32 {
            let item_offset = item.wrapping_mul(4);
            let lower = read_u32(table.wrapping_add(item_offset));
            if value < lower {
                continue;
            }
            let upper = read_u32(table.wrapping_add(UPPER_BOUNDS).wrapping_add(item_offset));
            if value < upper {
                let delta = read_u32(table.wrapping_add(DELTAS).wrapping_add(item_offset));
                write_u32(value_address, value.wrapping_add(delta));
                return;
            }
        }
    }
});
