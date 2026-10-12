// original: 0x00D5F500 physics_surface_material_index_from_name

/// Return the index of a physics-surface name in its relocated name table.
///
/// The cdecl argument is a NUL-terminated byte string. The function compares
/// it byte-for-byte with 42 entries of 64 bytes starting at the named image
/// table, returning the first matching zero-based index or `u32::MAX` when no
/// entry matches. The comparison is case-sensitive. Table bytes are read from
/// the relocated image at run time instead of being copied into this rewrite.
lf_checker_rt::export!(cdecl, rw_physics_surface_material_index_from_name(input_ptr: u32) -> u32 {
    const NAME_TABLE_VA: u32 = 0x010557A0;
    const NAME_COUNT: u32 = 42;
    const NAME_SLOT_SIZE: u32 = 0x40;

    #[inline(always)]
    unsafe fn read_byte(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }

    let table = lf_checker_rt::relocated(NAME_TABLE_VA);
    for name_index in 0..NAME_COUNT {
        let mut input = input_ptr;
        let mut candidate = table.wrapping_add(name_index.wrapping_mul(NAME_SLOT_SIZE));
        loop {
            let input_byte = unsafe { read_byte(input) };
            let table_byte = unsafe { read_byte(candidate) };
            if input_byte != table_byte {
                break;
            }
            if input_byte == 0 {
                return name_index;
            }
            input = input.wrapping_add(1);
            candidate = candidate.wrapping_add(1);
        }
    }
    u32::MAX
});
