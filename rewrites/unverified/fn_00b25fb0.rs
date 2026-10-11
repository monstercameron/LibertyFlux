// original: 0x00B25FB0 entity_replace_physics_reference

/// Replaces the entity's physics reference at `+0x1f8`. A non-null old
/// reference is released before the new pointer is stored. The byte at
/// `+0x1f4` is replaced by the second input byte with its low two bits set.
/// A non-null new reference is then acquired. Both helper calls receive the
/// address of the entity's reference field as their stack argument.
lf_checker_rt::export!(thiscall, rw_b25fb0(this: u32, replacement: u32, flags_byte: u32) -> () {
    const REFERENCE_FIELD: u32 = 0x1F8;
    const REFERENCE_FLAGS: u32 = 0x1F4;
    const REQUIRED_FLAGS: u8 = 0x03;
    const RELEASE_REFERENCE: u32 = 1;
    const ACQUIRE_REFERENCE: u32 = 2;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write_unaligned(value) }
    }

    unsafe {
        let field = this.wrapping_add(REFERENCE_FIELD);
        let old_reference = read_u32(field);
        if old_reference != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_REFERENCE, u32, old_reference, field);
        }

        let caller_flags = flags_byte as u8;
        write_u32(field, replacement);
        write_u8(this.wrapping_add(REFERENCE_FLAGS), caller_flags | REQUIRED_FLAGS);

        if replacement != 0 {
            let _ = lf_checker_rt::callee_thiscall!(ACQUIRE_REFERENCE, u32, replacement, field);
        }
    }
});
