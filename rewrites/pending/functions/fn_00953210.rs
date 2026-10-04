// original: 0x00953210 row_slot_or_null
/// Address the fixed-stride row for a 16-bit key, or null when out of range.
///
/// Keys above 0x5db have no row. Valid keys sign-extend (they are all
/// non-negative) and stride by 8 bytes from the row base.
export!(cdecl, rw_00953210(key: u32) -> u32 {
    let narrow = (key & 0xFFFF) as u16;
    if narrow > 0x5DB {
        return 0;
    }
    let signed = (narrow as i16) as i32;
    (relocated(0x11F7110) as i32).wrapping_add(signed.wrapping_mul(8)) as u32
});
