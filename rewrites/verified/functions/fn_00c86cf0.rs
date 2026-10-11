// original: 0x00c86cf0 fn_00c86cf0_vector_records_equal

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    let bits = unsafe { read_u32(lf_checker_rt::relocated(file_va)) };
    f32::from_bits(bits)
}

/// Return one when any of twelve corresponding float fields differs or is
/// unordered, and zero when every compared field is ordered and equal. The
/// object is in ECX, the other record is the single stack argument, and the
/// original returns its boolean in AL while cleaning that argument.
const VECTOR_COMPARE_OFFSETS: [u32; 12] = [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];

unsafe fn vector_records_equal(left: u32, right: u32, compare_last: bool) -> u8 {
    let length = if compare_last { VECTOR_COMPARE_OFFSETS.len() } else { VECTOR_COMPARE_OFFSETS.len() - 1 };
    for offset in VECTOR_COMPARE_OFFSETS.iter().take(length) {
        let a = f32::from_bits(unsafe { read_u32(left + offset) });
        let b = f32::from_bits(unsafe { read_u32(right + offset) });
        if a != b {
            return 1;
        }
    }
    0
}

lf_checker_rt::export!(thiscall, rw_00c86cf0_vector_records_equal(left: u32, right: u32) -> u8 {
    unsafe { vector_records_equal(left, right, true) }
});
