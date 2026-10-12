// original: 0x005C40F0 ped_factory_allocate_next_slot

/// Claims the next available slot in the process-wide resource pool. It scans
/// from the index after the stored cursor, wraps at the signed-count boundary,
/// and chooses the first flag byte with bit 7 set. The claim clears that bit,
/// advances the low-seven-bit tag with a minimum value of one, updates the
/// cursor and active count, and returns `base + index * stride`. If every
/// slot is occupied it returns zero without changing the manager.
lf_checker_rt::export!(cdecl, rw_005c40f0() -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x018b6f20;
        const BASE_POINTER: u32 = 0x00;
        const FLAGS_POINTER: u32 = 0x04;
        const SLOT_COUNT: u32 = 0x08;
        const SLOT_STRIDE: u32 = 0x0c;
        const CURSOR_INDEX: u32 = 0x10;
        const ACTIVE_COUNT: u32 = 0x14;
        const AVAILABLE_MASK: u8 = 0x80;

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

        let manager = lf_checker_rt::global::<u32>(POOL_GLOBAL).read_unaligned();
        let base = read_word(manager + BASE_POINTER);
        let flags = read_word(manager + FLAGS_POINTER);
        let count = read_word(manager + SLOT_COUNT);
        let stride = read_word(manager + SLOT_STRIDE);
        let current = read_word(manager + CURSOR_INDEX);
        let mut index = current.wrapping_add(1);
        if index == count {
            index = 0;
        }

        for _ in 0..count {
            let flag_address = flags.wrapping_add(index);
            let prior = read_byte(flag_address);
            if prior & AVAILABLE_MASK != 0 {
                let incremented_tag = (prior & 0x7f).wrapping_add(1) & 0x7f;
                let next_tag = if incremented_tag <= 1 { 1 } else { incremented_tag };
                write_byte(flag_address, next_tag);
                write_word(manager + CURSOR_INDEX, index);
                let active_count = read_word(manager + ACTIVE_COUNT);
                write_word(manager + ACTIVE_COUNT, active_count.wrapping_add(1));
                return base.wrapping_add(index.wrapping_mul(stride));
            }
            index += 1;
            if index == count {
                index = 0;
            }
        }
        0
    }
});
