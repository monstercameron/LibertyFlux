

lf_checker_rt::export!(thiscall, rw_005fb8b0(this: u32) -> u32 {
    // Mutant: change the resource word by one after the original's null checks.
    unsafe {
        let resource = read_u32(this.wrapping_add(0x64));
        if resource == 0 { return 0; }
        let referenced = read_u32(this.wrapping_add(0x5c));
        let object = if referenced == 0 { read_u32(this.wrapping_add(0x6c)) }
            else if read_u32(referenced.wrapping_add(8)) & 0x3000_0000 == 0x3000_0000 { read_u32(referenced) }
            else { 0 };
        if object == 0 { 0 } else { read_u32(resource.wrapping_add(0x160)).wrapping_add(1) }
    }
});

// Return the resource's word at offset 0x160 when both resource and type resolve.
lf_checker_rt::export!(thiscall, lane_base_005fb8b0(this: u32) -> u32 {
    const RESOURCE_POINTER: u32 = 0x64;
    const REFERENCED_TYPE: u32 = 0x5c;
    const FALLBACK_OBJECT: u32 = 0x6c;
    const TYPE_FLAGS: u32 = 0x08;
    const TYPE_MASK: u32 = 0x3000_0000;
    const RESOURCE_VALUE: u32 = 0x160;

    unsafe {
        let resource = read_u32(this.wrapping_add(RESOURCE_POINTER));
        if resource == 0 {
            return 0;
        }
        let referenced = read_u32(this.wrapping_add(REFERENCED_TYPE));
        let object = if referenced == 0 {
            read_u32(this.wrapping_add(FALLBACK_OBJECT))
        } else if read_u32(referenced.wrapping_add(TYPE_FLAGS)) & TYPE_MASK == TYPE_MASK {
            read_u32(referenced)
        } else {
            0
        };
        if object == 0 { 0 } else { read_u32(resource.wrapping_add(RESOURCE_VALUE)) }
    }
});

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
