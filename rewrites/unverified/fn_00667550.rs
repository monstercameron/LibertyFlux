// original: 0x00667550 rage::ptxTimeLine::~ptxTimeLine

/// Destroy the timeline's non-null entries in reverse array order. The object
/// holds the entry-array pointer at `+0x10`, a 16-bit entry count at `+0x14`,
/// and the timeline vtable at `+0`. Each entry's slot-zero thiscall receives
/// the entry object and a `1` flag. If the array pointer is non-null, release
/// it through allocator vtable slot `+0x0c` on the object in TLS slot zero.
/// Finally clear the array pointer and count, set the sentinel at `+0x1c` to
/// `u32::MAX`, and return zero. The original contains a second release call
/// behind a branch made unreachable by its preceding zeroing instruction.
lf_checker_rt::export!(thiscall, rw_00667550(this: u32) -> u32 {
    unsafe {
        const TIMELINE_VTABLE_FILE_VA: u32 = 0x00fe266c;
        const ENTRY_ARRAY_OFFSET: u32 = 0x10;
        const ENTRY_COUNT_OFFSET: u32 = 0x14;
        const SENTINEL_OFFSET: u32 = 0x1c;
        const ALLOCATOR_TLS_OFFSET: u32 = 8;
        const ALLOCATOR_RELEASE_SLOT: u32 = 0x0c;
        const ENTRY_VTABLE_SLOT: u32 = 0;
        const POINTER_SIZE: u32 = 4;
        const DESTROY_FLAG: u32 = 1;
        const SENTINEL: u32 = u32::MAX;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        write_word(this, lf_checker_rt::relocated(TIMELINE_VTABLE_FILE_VA));

        let array_pointer = read_word(this + ENTRY_ARRAY_OFFSET);
        let entry_count = (this + ENTRY_COUNT_OFFSET as u32) as *const u16;
        let entry_count = unsafe { entry_count.read_unaligned() } as u32;

        for index in (0..entry_count).rev() {
            let entry_address = array_pointer
                .wrapping_add(index.wrapping_mul(POINTER_SIZE));
            let entry_this = read_word(entry_address);
            if entry_this != 0 {
                let entry_vtable = read_word(entry_this);
                let destructor_entry = read_word(entry_vtable + ENTRY_VTABLE_SLOT);
                let destroy_entry: unsafe extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(destructor_entry as usize);
                let _ = destroy_entry(entry_this, DESTROY_FLAG);
            }
        }

        if array_pointer != 0 {
            let tls_base = lf_checker_rt::tls_slot(0);
            let allocator_object = read_word(tls_base + ALLOCATOR_TLS_OFFSET);
            let allocator_vtable = read_word(allocator_object);
            let release_entry = read_word(allocator_vtable + ALLOCATOR_RELEASE_SLOT);
            let release_array: unsafe extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(release_entry as usize);
            let _ = release_array(allocator_object, array_pointer);
        }

        write_word(this + ENTRY_ARRAY_OFFSET, 0);
        write_word(this + ENTRY_COUNT_OFFSET, 0);
        write_word(this + SENTINEL_OFFSET, SENTINEL);
        0
    }
});
