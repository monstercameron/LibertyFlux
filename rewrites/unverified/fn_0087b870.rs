// original: 0x0087B870 find_or_create_capture_node

/// Find an existing capture node or allocate and construct one.
///
/// `request` holds the existing-node pointer at `+0x20`. A non-null value is
/// passed to the locked lookup helper with node type `0x14`; a returned node
/// is reused. Otherwise a 0x28-byte, 0x10-aligned node is allocated through
/// TLS slot 0's allocator table and passed to the capture constructor. The
/// resulting node receives the request pointer at `+0x08` and type `0x14` at
/// `+0x06`. An allocation failure skips construction and faults on the common
/// field writes.
///
/// Original: cdecl with one stack pointer; lookup, allocator and constructor
/// calls are intercepted.
lf_checker_rt::export!(cdecl, rw_0087b870(request: u32) -> u32 {
    const EXISTING_NODE_OFFSET: u32 = 0x20;
    const NODE_OWNER_OFFSET: u32 = 0x08;
    const NODE_TYPE_OFFSET: u32 = 0x06;
    const TLS_SLOT: usize = 0;
    const ALLOCATOR_OFFSET: u32 = 0x08;
    const ALLOCATE_VTABLE_SLOT: u32 = 0x08;
    const CAPTURE_NODE_TYPE: u32 = 0x14;
    const ALLOCATION_SIZE: u32 = 0x28;
    const ALLOCATION_ALIGNMENT: u32 = 0x10;
    const LOOKUP_CAPTURE_NODE: u32 = 1;
    const CONSTRUCT_CAPTURE_NODE: u32 = 3;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let existing_node = read_u32(request.wrapping_add(EXISTING_NODE_OFFSET));
        let mut node = 0u32;
        if existing_node != 0 {
            node = lf_checker_rt::callee_thiscall!(
                LOOKUP_CAPTURE_NODE,
                u32,
                existing_node,
                CAPTURE_NODE_TYPE
            );
        }

        if node == 0 {
            let tls_anchor = lf_checker_rt::tls_slot(TLS_SLOT);
            let allocator = read_u32(tls_anchor.wrapping_add(ALLOCATOR_OFFSET));
            let allocator_vtable = read_u32(allocator);
            let allocate_target = read_u32(allocator_vtable.wrapping_add(ALLOCATE_VTABLE_SLOT));
            let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(allocate_target as usize);
            let allocated_node = allocate(allocator, ALLOCATION_SIZE, ALLOCATION_ALIGNMENT, 0);
            if allocated_node != 0 {
                node = lf_checker_rt::callee_thiscall!(
                    CONSTRUCT_CAPTURE_NODE,
                    u32,
                    allocated_node
                );
            }
        }

        write_u32(node.wrapping_add(NODE_OWNER_OFFSET), request);
        (node.wrapping_add(NODE_TYPE_OFFSET) as *mut u16).write_unaligned(CAPTURE_NODE_TYPE as u16);
        node
    }
});
