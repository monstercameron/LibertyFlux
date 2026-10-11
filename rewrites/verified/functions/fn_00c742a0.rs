// original: 0x00C742A0 native_get_ped_type_name

/// Return the table pointer for a zero-based entry in the 23-name ped-type
/// table. The bound is unsigned: indices greater than 22 return null, while
/// valid indices return the pointer stored in the relocated data table.
lf_checker_rt::export!(cdecl, rw_00c742a0(index: u32) -> u32 {
    const LAST_VALID_INDEX: u32 = 0x16;
    const NAME_POINTERS: u32 = 0x0104_b6f8;
    if index > LAST_VALID_INDEX {
        return 0;
    }
    unsafe {
        let table = lf_checker_rt::relocated(NAME_POINTERS);
        ((table.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned()
    }
});
