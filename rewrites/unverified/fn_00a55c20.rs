// original: 0x00A55C20 composite_bound_reset_all

/// Call the reset helper once for the owner, then once for every record in the four groups in order. Counts are signed and nonpositive counts skip their group. The routine has no defined EAX result.
///
/// The owner uses the original 32-bit layout. Each group tuple names the
/// signed count offset, record base, record stride and flag-byte base. The
/// entry uses thiscall; record helpers receive their selected record in ECX.
lf_checker_rt::export!(thiscall, rw_composite_bound_reset_all(this: u32) -> u32 {
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
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    let reset_groups: [(u32, u32, u32); 4] = [
        (0x1954, 0x1690, 0xc0),
        (0x1950, 0x90, 0xb0),
        (0x1958, 0x1750, 0xe0),
        (0x195c, 0x1830, 0x120),
    ];
    let mut group_index = 0usize;
    while group_index < reset_groups.len() {
        let (count_offset, record_offset, stride) = reset_groups[group_index];
        let count = unsafe { read_i32(this.wrapping_add(count_offset)) };
        let mut index = 0i32;
        while index < count {
            let record = this.wrapping_add(record_offset).wrapping_add((index as u32).wrapping_mul(stride));
            let _ = lf_checker_rt::callee_thiscall!(1, u32, record);
            index += 1;
        }
        group_index += 1;
    }
    0
});
