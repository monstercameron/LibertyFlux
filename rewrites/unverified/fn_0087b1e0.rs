// original: 0x0087B1E0 rage::crmtRequestAddSubtract::create_node

/// Find or allocate the add/subtract node owned by a request.
///
/// `request` is a pointer to a request object whose existing-node pointer is
/// at `+0x20`. A non-null pointer is passed to the locked lookup helper with
/// node type 6. If lookup returns null, the function allocates a 0x30-byte,
/// 0x10-aligned node from TLS slot 0's allocator table at slot `+0x08`, then
/// calls the base initializer with type 6. Whether reused or allocated, the
/// newly allocated node receives the add/subtract vtable. Both new and reused
/// nodes receive the request pointer at `+0x08` and the 16-bit type at `+0x06`;
/// the node pointer is returned.
///
/// Original: cdecl with one stack pointer. The lookup, allocator and base
/// initializer are intercepted; the allocator is reached through its table.
lf_checker_rt::export!(cdecl, rw_0087b1e0(request: u32) -> u32 {
    const EXISTING_NODE_OFFSET: u32 = 0x20;
    const TLS_SLOT: usize = 0;
    const ALLOCATOR_OFFSET: u32 = 0x08;
    const ALLOCATOR_VTABLE_SLOT: u32 = 0x08;
    const NODE_VTABLE: u32 = 0x00FE837C;
    const NODE_TYPE: u32 = 6;
    const ALLOCATION_SIZE: u32 = 0x30;
    const ALLOCATION_ALIGNMENT: u32 = 0x10;
    const LOOKUP_NODE: u32 = 1;
    const INITIALIZE_NODE: u32 = 3;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let existing = read_u32(request.wrapping_add(EXISTING_NODE_OFFSET));
        let mut node = 0u32;
        if existing != 0 {
            node = lf_checker_rt::callee_thiscall!(LOOKUP_NODE, u32, existing, NODE_TYPE);
        }

        if node == 0 {
            let tls_anchor = lf_checker_rt::tls_slot(TLS_SLOT);
            let allocator = read_u32(tls_anchor.wrapping_add(ALLOCATOR_OFFSET));
            let allocator_vtable = read_u32(allocator);
            let allocate_target = read_u32(allocator_vtable.wrapping_add(ALLOCATOR_VTABLE_SLOT));
            let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(allocate_target as usize);
            node = allocate(allocator, ALLOCATION_SIZE, ALLOCATION_ALIGNMENT, 0);
            let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_NODE, u32, node, NODE_TYPE);
            (node as *mut u32).write_unaligned(lf_checker_rt::relocated(NODE_VTABLE));
        }

        (node.wrapping_add(0x06) as *mut u16).write_unaligned(NODE_TYPE as u16);
        write_u32(node, 0x08, request);
        node
    }
});
