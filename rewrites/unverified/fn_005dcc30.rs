// original: 0x005DCC30 advance_ped_factory_slot

/// Claim the next available entry in the pedestrian factory slot table.
///
/// The manager at `FACTORY_MANAGER_SLOT` supplies the entry base, byte-flag
/// array, entry count, stride, current index and outstanding-allocation
/// count. Starting after the current index, the search wraps at most once
/// and accepts the first flag with bit 7 set. It clears that marker, advances
/// the flag's low-seven-bit use count (wrapping zero or one to one), stores
/// the selected index, increments the outstanding count and returns the
/// selected entry address. If no marker is available, it returns zero.
/// Pointer arithmetic and counters wrap as 32-bit unsigned values.
lf_checker_rt::export!(cdecl, rw_005dcc30() -> u32 {
    const FACTORY_MANAGER_SLOT: u32 = 0x018B6F0C;
    const ENTRY_BASE_OFFSET: u32 = 0;
    const FLAGS_BASE_OFFSET: u32 = 4;
    const ENTRY_COUNT_OFFSET: u32 = 8;
    const ENTRY_STRIDE_OFFSET: u32 = 12;
    const CURRENT_INDEX_OFFSET: u32 = 16;
    const OUTSTANDING_COUNT_OFFSET: u32 = 20;
    const ENTRY_AVAILABLE: u8 = 0x80;
    const USE_COUNT_MASK: u8 = 0x7F;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    let manager_slot = lf_checker_rt::relocated(FACTORY_MANAGER_SLOT);
    let manager = unsafe { read_u32(manager_slot) };
    let entry_base = unsafe { read_u32(manager.wrapping_add(ENTRY_BASE_OFFSET)) };
    let flags_base = unsafe { read_u32(manager.wrapping_add(FLAGS_BASE_OFFSET)) };
    let entry_count = unsafe { read_u32(manager.wrapping_add(ENTRY_COUNT_OFFSET)) };
    let entry_stride = unsafe { read_u32(manager.wrapping_add(ENTRY_STRIDE_OFFSET)) };
    let current_index = unsafe { read_u32(manager.wrapping_add(CURRENT_INDEX_OFFSET)) };
    let mut candidate = current_index.wrapping_add(1);
    let mut wrapped = false;

    loop {
        if candidate == entry_count {
            candidate = 0;
            if wrapped {
                return 0;
            }
            wrapped = true;
        }

        let flag_address = flags_base.wrapping_add(candidate);
        let flag = unsafe { read_u8(flag_address) };
        if flag & ENTRY_AVAILABLE == 0 {
            candidate = candidate.wrapping_add(1);
            continue;
        }

        let old_use_count = flag & USE_COUNT_MASK;
        let incremented_use_count = old_use_count.wrapping_add(1) & USE_COUNT_MASK;
        let new_use_count = if incremented_use_count <= 1 {
            1
        } else {
            incremented_use_count
        };
        unsafe { write_u8(flag_address, new_use_count) };
        let outstanding = unsafe {
            read_u32(manager.wrapping_add(OUTSTANDING_COUNT_OFFSET))
        };
        unsafe {
            write_u32(
                manager.wrapping_add(OUTSTANDING_COUNT_OFFSET),
                outstanding.wrapping_add(1),
            );
            write_u32(manager.wrapping_add(CURRENT_INDEX_OFFSET), candidate);
        }
        return entry_base.wrapping_add(candidate.wrapping_mul(entry_stride));
    }
});
