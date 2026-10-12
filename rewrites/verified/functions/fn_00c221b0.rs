// original: 0x00C221B0 initialize_file_sphere_record (proposed)
/// Initialize a compact sphere-like record from a three-word source vector,
/// a radius, an identifier and one flag byte. The low flag bit is copied to
/// bit 1 while the destination's two low reserved bits are preserved as the
/// original mask specifies; bit 0 is then set. The radius and its single
/// precision square are stored separately, and the image-global value is
/// copied into the record. `this` is the destination in ECX, followed by four
/// stack words: source pointer, radius bits, identifier and flag byte.
lf_checker_rt::export!(thiscall, rw_c221b0(this: u32, source: u32, radius_bits: u32, identifier: u32, flags: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x00;
        const GLOBAL_VALUE: u32 = 0x04;
        const IDENTIFIER: u32 = 0x08;
        const SOURCE_VECTOR: u32 = 0x0c;
        const RADIUS: u32 = 0x18;
        const RADIUS_SQUARED: u32 = 0x1c;
        const FLAGS_KEEP: u32 = 0xffff_ffc1;
        const IMAGE_GLOBAL_VA: u32 = 0x0117_35b4;
        let old_flags = (this.wrapping_add(FLAGS) as *const u32).read_unaligned();
        let new_flags = (old_flags & FLAGS_KEEP) | (((flags as u8 as u32) & 1) << 1) | 1;
        (this.wrapping_add(FLAGS) as *mut u32).write_unaligned(new_flags);

        let source_words = source as *const u32;
        let source_x = source_words.read_unaligned();
        (this.wrapping_add(SOURCE_VECTOR) as *mut u32).write_unaligned(source_x);
        let source_y = source_words.add(1).read_unaligned();
        (this.wrapping_add(SOURCE_VECTOR + 4) as *mut u32).write_unaligned(source_y);
        let source_z = source_words.add(2).read_unaligned();
        (this.wrapping_add(SOURCE_VECTOR + 8) as *mut u32).write_unaligned(source_z);

        let radius = f32::from_bits(radius_bits);
        let radius_squared = core::hint::black_box(radius) * core::hint::black_box(radius);
        (this.wrapping_add(RADIUS) as *mut u32).write_unaligned(radius_bits);
        (this.wrapping_add(RADIUS_SQUARED) as *mut u32).write_unaligned(radius_squared.to_bits());
        let image_value = lf_checker_rt::global::<u32>(IMAGE_GLOBAL_VA).read_unaligned();
        (this.wrapping_add(GLOBAL_VALUE) as *mut u32).write_unaligned(image_value);
        (this.wrapping_add(IDENTIFIER) as *mut u32).write_unaligned(identifier);
        0
    }
});
