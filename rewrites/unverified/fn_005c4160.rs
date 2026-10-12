// original: 0x005C4160 ped_tls_pool_descriptor_init_c

/// Builds a pool descriptor from two allocator results. `this` is an
/// ECX pointer to the output descriptor; three stack words are ignored and
/// cleaned by the thiscall return. TLS slot 0 points to an allocator object
/// whose virtual slot at +8 allocates the large block and the byte-tag array.
/// The descriptor receives both pointers, the requested tag-byte count, the
/// tag-array length, `u32::MAX` at +0x10, zero at +0x14 and one at +0x18.
/// Every tag byte is initialized to `(old & 0x81) | 0x81`.
lf_checker_rt::export!(thiscall, rw_005c4160(this: u32, _arg0: u32, _arg1: u32, _arg2: u32) -> u32 {
    unsafe {
        const DESCRIPTOR_LARGE_BLOCK: u32 = 0x00;
        const DESCRIPTOR_TAG_ARRAY: u32 = 0x04;
        const DESCRIPTOR_TAG_BYTES: u32 = 0x08;
        const DESCRIPTOR_TAG_CAPACITY: u32 = 0x0c;
        const DESCRIPTOR_SENTINEL: u32 = 0x10;
        const DESCRIPTOR_COUNT: u32 = 0x14;
        const DESCRIPTOR_ACTIVE: u32 = 0x18;
        const TLS_ALLOCATOR_OFFSET: u32 = 0x08;
        const ALLOCATOR_VTABLE_OFFSET: u32 = 0x00;
        const ALLOCATOR_SLOT: u32 = 0x08;
        const LARGE_ALLOCATION: u32 = 0x70080;
        const TAG_ARRAY_ALLOCATION: u32 = 0x78;
        const DESCRIPTOR_TAG_LENGTH: u32 = 0x78;
        const POOL_ALIGNMENT: u32 = 0x10;
        const TAG_MASK: u8 = 0x81;

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
        #[inline(always)]
        unsafe fn allocator_call(target: u32, allocator: u32, bytes: u32) -> u32 {
            let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            allocate(allocator, bytes, POOL_ALIGNMENT, 0)
        }

        let tls_context = lf_checker_rt::tls_slot(0);
        let allocator = read_word(tls_context + TLS_ALLOCATOR_OFFSET);
        let allocator_vtable = read_word(allocator + ALLOCATOR_VTABLE_OFFSET);
        let allocate_target = read_word(allocator_vtable + ALLOCATOR_SLOT);
        let large_block = allocator_call(allocate_target, allocator, LARGE_ALLOCATION);
        let tag_array = allocator_call(allocate_target, allocator, TAG_ARRAY_ALLOCATION);

        write_word(this + DESCRIPTOR_LARGE_BLOCK, large_block);
        write_word(this + DESCRIPTOR_TAG_ARRAY, tag_array);
        write_word(this + DESCRIPTOR_TAG_BYTES, DESCRIPTOR_TAG_LENGTH);
        write_byte(this + DESCRIPTOR_ACTIVE, 1);
        write_word(this + DESCRIPTOR_TAG_CAPACITY, DESCRIPTOR_TAG_LENGTH);
        write_word(this + DESCRIPTOR_SENTINEL, u32::MAX);
        write_word(this + DESCRIPTOR_COUNT, 0);

        for offset in 0..DESCRIPTOR_TAG_LENGTH {
            let old = read_byte(tag_array + offset);
            write_byte(tag_array + offset, (old & TAG_MASK) | TAG_MASK);
        }
        this
    }
});
