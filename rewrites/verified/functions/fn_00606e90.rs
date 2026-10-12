// original: 0x00606e90 streaming_initialize_type_one_payload

/// Initialises a type-one streaming payload and copies its three-word source
/// vector into the object's value slot. The source is read in order after the
/// identity defaults are stored, so a source pointer that aliases the object
/// observes those writes. The object is addressed through the 32-bit checker
/// heap representation; the vector words are copied without float conversion.
lf_checker_rt::export!(thiscall, rw_00606e90(this: u32, source: u32) -> u32 {
    const STATE: u32 = 0x74;
    const MATRIX_X: u32 = 0xE0;
    const MATRIX_Y: u32 = 0xF4;
    const MATRIX_Z: u32 = 0x108;
    const VALUE_X: u32 = 0x110;
    const VALUE_Y: u32 = 0x114;
    const VALUE_Z: u32 = 0x118;
    unsafe {
        (this.wrapping_add(STATE) as *mut u32).write_unaligned(1);
        (this.wrapping_add(0x88) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x8C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x90) as *mut u32).write_unaligned(0);
        (this.wrapping_add(MATRIX_X) as *mut u32).write_unaligned(0x3F80_0000);
        (this.wrapping_add(0xE4) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0xE8) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0xF0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(MATRIX_Y) as *mut u32).write_unaligned(0x3F80_0000);
        (this.wrapping_add(0xF8) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x100) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x104) as *mut u32).write_unaligned(0);
        (this.wrapping_add(MATRIX_Z) as *mut u32).write_unaligned(0x3F80_0000);
        (this.wrapping_add(VALUE_Z) as *mut u32).write_unaligned(0);
        (this.wrapping_add(VALUE_Y) as *mut u32).write_unaligned(0);
        (this.wrapping_add(VALUE_X) as *mut u32).write_unaligned(0);
        let x = (source as *const u32).read_unaligned();
        (this.wrapping_add(VALUE_X) as *mut u32).write_unaligned(x);
        let y = (source.wrapping_add(4) as *const u32).read_unaligned();
        (this.wrapping_add(VALUE_Y) as *mut u32).write_unaligned(y);
        let z = (source.wrapping_add(8) as *const u32).read_unaligned();
        (this.wrapping_add(VALUE_Z) as *mut u32).write_unaligned(z);
        z
    }
});
