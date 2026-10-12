// original: 0x0087B1A0 rage::crmtNodeAddSubtract::vf0

/// Run the add/subtract node's deleting destructor.
///
/// `this` is the node object and `delete_flag` is the incoming stack word.
/// The destructor installs the plain node vtable at `this+0`, runs the pair
/// teardown and parent teardown helpers in order, and, when bit 0 of
/// `delete_flag` is set, frees the node through TLS slot 0's allocator table
/// slot `+0x0c`. The free call uses the allocator as ECX and `this` as its one
/// stack argument. The function returns `this`.
///
/// Original: thiscall with one stack word; both direct helpers and the
/// TLS-resolved free function are intercepted by the checker.
lf_checker_rt::export!(thiscall, rw_0087b1a0(this: u32, delete_flag: u32) -> u32 {
    const NODE_VTABLE: u32 = 0x00FE8504;
    const TLS_SLOT: usize = 0;
    const ALLOCATOR_OFFSET: u32 = 0x08;
    const ALLOCATOR_VTABLE_SLOT: u32 = 0x0c;
    const PAIR_TEARDOWN: u32 = 1;
    const PARENT_TEARDOWN: u32 = 2;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(NODE_VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(PAIR_TEARDOWN, u32, this);
        let _ = lf_checker_rt::callee_thiscall!(PARENT_TEARDOWN, u32, this);

        if delete_flag & 1 != 0 {
            let tls_anchor = lf_checker_rt::tls_slot(TLS_SLOT);
            let allocator = read_u32(tls_anchor.wrapping_add(ALLOCATOR_OFFSET));
            let allocator_vtable = read_u32(allocator);
            let free_target = read_u32(allocator_vtable.wrapping_add(ALLOCATOR_VTABLE_SLOT));
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(free_target as usize);
            let _ = free(allocator, this);
        }
        this
    }
});
