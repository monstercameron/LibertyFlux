// original: 0x0087B310 rage::crmtIterator::deleting_destructor

/// Run the iterator's deleting destructor.
///
/// `this` is the iterator object and `delete_flag` is the incoming stack word.
/// The destructor installs the iterator vtable, then frees the object through
/// TLS slot 0's allocator vtable slot `+0x0c` when bit 0 of `delete_flag` is
/// set. It returns `this` in either case.
///
/// Original: thiscall with one stack word; the TLS-resolved free call is
/// intercepted by the checker.
lf_checker_rt::export!(thiscall, rw_0087b310(this: u32, delete_flag: u32) -> u32 {
    const ITERATOR_VTABLE: u32 = 0x00FE83BC;
    const TLS_SLOT: usize = 0;
    const ALLOCATOR_OFFSET: u32 = 0x08;
    const FREE_VTABLE_SLOT: u32 = 0x0C;
    const FREE_CALL: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(ITERATOR_VTABLE));
        if delete_flag & 1 != 0 {
            let tls_anchor = lf_checker_rt::tls_slot(TLS_SLOT);
            let allocator = read_u32(tls_anchor.wrapping_add(ALLOCATOR_OFFSET));
            let allocator_vtable = read_u32(allocator);
            let free_target = read_u32(allocator_vtable.wrapping_add(FREE_VTABLE_SLOT));
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(free_target as usize);
            let _ = free(allocator, this);
        }
        this
    }
});
