// original: 0x00A52FE0 sum_flagged_query_values

/// For each group in order, sum the unsigned 32-bit helper results for records whose flag byte is nonzero. Counts are signed and nonpositive counts skip their group. Addition wraps at 32 bits, and the sum is returned in EAX.
///
/// The owner uses the original 32-bit layout. Each group tuple names the
/// signed count offset, record base, record stride and flag-byte base. The
/// entry uses thiscall; record helpers receive their selected record in ECX.
lf_checker_rt::export!(thiscall, rw_sum_flagged_query_values(this: u32) -> u32 {
    const GROUPS: [(u32, u32, u32, u32); 4] = [
        (0x1950, 0x90, 0xb0, 0x1960),
        (0x1954, 0x1690, 0xc0, 0x1980),
        (0x1958, 0x1750, 0xe0, 0x1981),
        (0x195c, 0x1830, 0x120, 0x1982),
    ];
    #[inline(always)]
    unsafe fn read_i32(address: u32) -> i32 {
        unsafe { (address as *const i32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    let mut total = 0u32;
    let mut group_index = 0usize;
    while group_index < GROUPS.len() {
        let (count_offset, record_offset, stride, flag_offset) = GROUPS[group_index];
        let count = unsafe { read_i32(this.wrapping_add(count_offset)) };
        let mut index = 0i32;
        while index < count {
            let flag = unsafe { read_u8(this.wrapping_add(flag_offset).wrapping_add(index as u32)) };
            if flag != 0 {
                let record = this.wrapping_add(record_offset).wrapping_add((index as u32).wrapping_mul(stride));
                let value = lf_checker_rt::callee_thiscall!(1, u32, record);
                total = total.wrapping_add(value);
            }
            index += 1;
        }
        group_index += 1;
    }
    total
});
