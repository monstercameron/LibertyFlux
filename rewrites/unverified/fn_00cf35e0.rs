// original: 0x00CF35E0 stunt_jump_contains_point
/// Test whether a three-float point lies inside the receiver's inclusive axis-aligned bounds. Minimum coordinates are at offsets 0, 4 and 8; maxima are at 0x10, 0x14 and 0x18. Unordered comparisons such as NaN fail the containment test.
lf_checker_rt::export!(thiscall, rw_00cf35e0(this: u32, point: u32) -> u8 {
    const MIN_X_OFFSET: u32 = 0x00;
    const MIN_Y_OFFSET: u32 = 0x04;
    const MIN_Z_OFFSET: u32 = 0x08;
    const MAX_X_OFFSET: u32 = 0x10;
    const MAX_Y_OFFSET: u32 = 0x14;
    const MAX_Z_OFFSET: u32 = 0x18;
    let minimum_x = unsafe { (this.wrapping_add(MIN_X_OFFSET) as *const f32).read_unaligned() };
    let minimum_y = unsafe { (this.wrapping_add(MIN_Y_OFFSET) as *const f32).read_unaligned() };
    let minimum_z = unsafe { (this.wrapping_add(MIN_Z_OFFSET) as *const f32).read_unaligned() };
    let maximum_x = unsafe { (this.wrapping_add(MAX_X_OFFSET) as *const f32).read_unaligned() };
    let maximum_y = unsafe { (this.wrapping_add(MAX_Y_OFFSET) as *const f32).read_unaligned() };
    let maximum_z = unsafe { (this.wrapping_add(MAX_Z_OFFSET) as *const f32).read_unaligned() };
    let point_x = unsafe { (point as *const f32).read_unaligned() };
    let point_y = unsafe { (point.wrapping_add(4) as *const f32).read_unaligned() };
    let point_z = unsafe { (point.wrapping_add(8) as *const f32).read_unaligned() };
    u8::from(point_x >= minimum_x && point_y >= minimum_y && point_z >= minimum_z
        && point_x <= maximum_x && point_y <= maximum_y && point_z <= maximum_z)
});
