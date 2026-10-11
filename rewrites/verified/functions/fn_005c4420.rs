// original: 0x005C4420 ped_task_resource_tag_predicate

/// Tests a ped-related flag and a nested resource tag. Flag bit 2 at offset
/// 0x26c succeeds immediately. Otherwise the low nibble at 0x1e2 must be at
/// least 2, the pointer at 0x1bc must be non-null, and the pointed-to word at
/// offset 0x28 must have masked value 0x80. The object pointer is in ECX and
/// the result is returned in AL.
lf_checker_rt::export!(thiscall, rw_005c4420(this: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x26c;
        const KIND: u32 = 0x1e2;
        const RESOURCE: u32 = 0x1bc;
        const RESOURCE_TAG: u32 = 0x28;
        const FLAG_MASK: u8 = 0x04;
        const KIND_MASK: u8 = 0x0f;
        const TAG_MASK: u32 = 0x3c0;
        const EXPECTED_TAG: u32 = 0x80;

        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        if read_byte(this + FLAGS) & FLAG_MASK != 0 {
            return 1;
        }
        if read_byte(this + KIND) & KIND_MASK < 2 {
            return 0;
        }
        let resource = read_word(this + RESOURCE);
        if resource == 0 {
            return 0;
        }
        u32::from(read_word(resource + RESOURCE_TAG) & TAG_MASK == EXPECTED_TAG)
    }
});
