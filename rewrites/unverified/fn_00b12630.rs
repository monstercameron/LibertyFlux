// original: 0x00B12630 reset_dynamic_entity_physics_fields

/// Move flag bit 5 into bit 6, preserve the other flag bits, and copy four
/// initialized single-precision constants into the object's physics vector at
/// offsets 0xe0 through 0xec. Constants are copied as raw bits so NaN payloads
/// and signed zero remain unchanged. This no-argument method uses thiscall.
lf_checker_rt::export!(thiscall, rw_00b12630(this: u32) -> () {
    const FLAGS: u32 = 0xF4;
    const COPY_SOURCES: [(u32, u32); 4] = [
        (0x01B4_B2A0, 0xE0),
        (0x01B4_B2A4, 0xE4),
        (0x01B4_B2A8, 0xE8),
        (0x01B4_B2AC, 0xEC),
    ];

    unsafe {
        let flag_address = this.wrapping_add(FLAGS);
        let flags = (flag_address as *const u8).read();
        let updated_flags = (flags & !0x60) | ((flags & 0x20) << 1);
        (flag_address as *mut u8).write(updated_flags);

        for (source_va, destination_offset) in COPY_SOURCES {
            let value = (lf_checker_rt::global::<u32>(source_va) as *const u32)
                .read_unaligned();
            (this.wrapping_add(destination_offset) as *mut u32).write_unaligned(value);
        }
        (this.wrapping_add(0xF0) as *mut u32).write_unaligned(0);
    }
});
