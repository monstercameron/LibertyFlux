// original: 0x00c561d0 initialize_event_numeric_fields
/// Initialize the numeric portion of an object. Zero eighteen dwords from offsets +0x24 through +0x68, set the listed limit fields to 25.0, and set the two final fields to the square of the immutable scalar read from the image. The function has no defined return value.
lf_checker_rt::export!(thiscall, rw_00c561d0(this: u32) -> u32 {
unsafe {
    const TWENTY_FIVE_BITS: u32 = 0x41c8_0000;
    const TWENTY_FIVE_FIELDS: [u32; 7] = [0x6c, 0x70, 0x74, 0x78, 0x80, 0x84, 0x88];
    const SOURCE_RADIUS_VA: u32 = 0x00ee_f27c;
    core::ptr::write_unaligned(this.wrapping_add(0x24) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x28) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x2c) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x30) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x34) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x38) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x3c) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x40) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x44) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x48) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x4c) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x50) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x54) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x58) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x5c) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x60) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x64) as *mut u32, 0);
    core::ptr::write_unaligned(this.wrapping_add(0x68) as *mut u32, 0);
    for offset in TWENTY_FIVE_FIELDS {
        core::ptr::write_unaligned(this.wrapping_add(offset) as *mut u32, TWENTY_FIVE_BITS);
    }
    let source_radius_bits = lf_checker_rt::global::<u32>(SOURCE_RADIUS_VA).read_unaligned();
    let source_radius = f32::from_bits(source_radius_bits);
    let radius_squared = (core::hint::black_box(source_radius) * core::hint::black_box(source_radius)).to_bits();
    core::ptr::write_unaligned(this.wrapping_add(0x7c) as *mut u32, radius_squared);
    core::ptr::write_unaligned(this.wrapping_add(0x8c) as *mut u32, radius_squared);
}
0
});
