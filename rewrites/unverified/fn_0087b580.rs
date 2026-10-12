// original: 0x0087B580 rage::crmtNodeFrame::~crmtNodeFrame__deleting

/// Run the frame node's deleting destructor.
///
/// `this` is a frame node and `delete_flag` is the incoming stack word. The
/// destructor always calls the child-state cleanup helper. It frees the node
/// through TLS slot 0's allocator table when bit 0 of `delete_flag` is set,
/// then returns `this`.
///
/// Original: thiscall with one stack word; cleanup and free calls are
/// intercepted by the checker.
lf_checker_rt::export!(thiscall, rw_0087b580(this: u32, delete_flag: u32) -> u32 {
    const TLS_SLOT: usize = 0;
    const ALLOCATOR_OFFSET: u32 = 0x08;
    const FREE_VTABLE_SLOT: u32 = 0x0C;
    const CLEAR_CHILD_STATE: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(CLEAR_CHILD_STATE, u32, this);
        if delete_flag & 1 != 0 && this != 0 {
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
