// original: 0x006954F0 id_table_init (proposed)

/// Initialize the global ID table when its 16-bit sentinel at image offset
/// `+0x15F2386` is zero. The first path requests a table through the
/// thread-local allocator's vtable slot two, stores the returned pointer, and
/// writes `14` to both 16-bit counters. When the sentinel is already set, the
/// function updates only the lower counter and returns `14`. The data-table
/// addresses are relocated image references; the allocator call is modeled
/// by a scripted stand-in.
lf_checker_rt::export!(thiscall, rw_006954F0(_context: u32, _unused_stack_value: u32) -> u32 {
    unsafe {
        const ID_TABLE_POINTER: u32 = 0x019F2380;
        const ID_TABLE_LENGTH: u32 = 0x019F2384;
        const ID_TABLE_INITIALIZED: u32 = 0x019F2386;
        const INITIAL_CAPACITY: u32 = 14;
        const ALLOCATE_TABLE_ID: u32 = 2;

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let initialized = lf_checker_rt::relocated(ID_TABLE_INITIALIZED);
        if unsafe { read_u16(initialized) } != 0 {
            unsafe { write_u16(lf_checker_rt::relocated(ID_TABLE_LENGTH), INITIAL_CAPACITY as u16) };
            return INITIAL_CAPACITY;
        }

        let table = unsafe {
            lf_checker_rt::callee_thiscall!(ALLOCATE_TABLE_ID, u32, 0, 56, 16, 0)
        };
        unsafe {
            write_u16(initialized, INITIAL_CAPACITY as u16);
            write_u16(lf_checker_rt::relocated(ID_TABLE_LENGTH), INITIAL_CAPACITY as u16);
            write_u32(lf_checker_rt::relocated(ID_TABLE_POINTER), table);
        }
        table
    }
});
