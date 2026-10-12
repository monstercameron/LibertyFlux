// original: 0x005C3510 CPedData::vf0

/// Installs the class vtable in the object and returns the same pointer. When
/// argument bit 0 is set, the method also indexes a manager bitmap using the
/// signed quotient `(this - range_base) / element_stride`, sets bit 7, lowers
/// the signed minimum index when needed, and decrements the signed active
/// count. The bounded proof sends bit 0 clear, so it observes only the vtable
/// write and return value.
lf_checker_rt::export!(thiscall, rw_005c3510(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00fe0aa4;
        const POOL_GLOBAL: u32 = 0x018b6f20;
        const RANGE_BASE: u32 = 0x00;
        const BITMAP_POINTER: u32 = 0x04;
        const ELEMENT_STRIDE: u32 = 0x0c;
        const MINIMUM_INDEX: u32 = 0x10;
        const ACTIVE_COUNT: u32 = 0x14;
        const ENABLE_TRACKING: u32 = 0x01;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_byte(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        write_word(this, lf_checker_rt::relocated(VTABLE_FILE_VA));
        if flags & ENABLE_TRACKING != 0 {
            let pool = lf_checker_rt::global::<u32>(POOL_GLOBAL).read_unaligned();
            let range_base = read_word(pool + RANGE_BASE);
            let bitmap = read_word(pool + BITMAP_POINTER);
            let divisor = read_word(pool + ELEMENT_STRIDE) as i32;
            if divisor != 0 {
                let displacement = this.wrapping_sub(range_base) as i32;
                let index = displacement / divisor;
                let flag_address = bitmap.wrapping_add(index as u32);
                write_byte(flag_address, read_byte(flag_address) | 0x80);

                let minimum_address = pool + MINIMUM_INDEX;
                let old_minimum = read_word(minimum_address) as i32;
                if index < old_minimum {
                    write_word(minimum_address, index as u32);
                }
                let count_address = pool + ACTIVE_COUNT;
                write_word(count_address, read_word(count_address).wrapping_sub(1));
            }
        }
        this
    }
});
