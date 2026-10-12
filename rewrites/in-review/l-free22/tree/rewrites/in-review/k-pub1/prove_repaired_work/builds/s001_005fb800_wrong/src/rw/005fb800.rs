lf_checker_rt::export!(thiscall, rw_005fb800(this: u32) -> u32 {
    // Mutant: ignore the type validity flags and return the referenced pointer.
    unsafe {
        let referenced = read_u32(this.wrapping_add(0x5c));
        if referenced == 0 {
            read_u32(this.wrapping_add(0x6c))
        } else {
            read_u32(referenced)
        }
    }
});

// Resolve the instance's referenced type only when its type flags are valid.
lf_checker_rt::export!(thiscall, lane_base_005fb800(this: u32) -> u32 {
    const REFERENCED_TYPE: u32 = 0x5c;
    const FALLBACK_OBJECT: u32 = 0x6c;
    const TYPE_FLAGS: u32 = 0x08;
    const TYPE_MASK: u32 = 0x3000_0000;

    unsafe {
        let referenced = read_u32(this.wrapping_add(REFERENCED_TYPE));
        if referenced == 0 {
            read_u32(this.wrapping_add(FALLBACK_OBJECT))
        } else if read_u32(referenced.wrapping_add(TYPE_FLAGS)) & TYPE_MASK == TYPE_MASK {
            read_u32(referenced)
        } else {
            0
        }
    }
});

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
