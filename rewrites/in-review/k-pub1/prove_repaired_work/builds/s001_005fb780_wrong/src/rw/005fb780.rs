lf_checker_rt::export!(thiscall, rw_005fb780(_this: u32) -> u8 {
    // Mutant: always reports an active item, including when every tested field is clear.
    1
});

// Return whether the resolved fragment object has any active referenced item.
lf_checker_rt::export!(thiscall, lane_base_005fb780(this: u32) -> u8 {
    const REFERENCED_TYPE: u32 = 0x5c;
    const FALLBACK_OBJECT: u32 = 0x6c;
    const TYPE_FLAGS: u32 = 0x08;
    const TYPE_MASK: u32 = 0x3000_0000;
    const OBJECT_READY: u32 = 0x1f9;
    const OBJECT_INDEX_COUNT: u32 = 0x1f3;
    const OBJECT_FIRST_COUNT: u32 = 0xdc;
    const OBJECT_SECOND_COUNT: u32 = 0xe0;
    const OBJECT_REFERENCES: u32 = 0xd4;
    const REFERENCE_TARGET: u32 = 0x98;
    const TARGET_COUNT: u32 = 0x08;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    unsafe {
        let referenced = read_u32(this.wrapping_add(REFERENCED_TYPE));
        let object = if referenced == 0 {
            read_u32(this.wrapping_add(FALLBACK_OBJECT))
        } else if read_u32(referenced.wrapping_add(TYPE_FLAGS)) & TYPE_MASK == TYPE_MASK {
            read_u32(referenced)
        } else {
            0
        };

        if read_u8(object.wrapping_add(OBJECT_READY)) != 0 {
            return 1;
        }
        if read_u16(object.wrapping_add(OBJECT_FIRST_COUNT)) > 0 {
            return 1;
        }
        if read_u32(object.wrapping_add(OBJECT_SECOND_COUNT)) != 0 {
            return 1;
        }

        let reference_count = u32::from(read_u8(object.wrapping_add(OBJECT_INDEX_COUNT)));
        let references = read_u32(object.wrapping_add(OBJECT_REFERENCES));
        for index in 0..reference_count {
            let item = read_u32(references.wrapping_add(index.wrapping_mul(4)));
            let target = read_u32(item.wrapping_add(REFERENCE_TARGET));
            if target != 0 && read_u16(target.wrapping_add(TARGET_COUNT)) > 0 {
                return 1;
            }
        }
        0
    }
});

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
