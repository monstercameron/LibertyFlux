// original: 0x0087B620 rage::crmtRequestFrame::create_node

/// Find or allocate the frame node owned by a request.
///
/// `request` stores an existing node pointer at `+0x20`. A non-null existing
/// pointer is passed to the locked lookup helper with signed node type 9. If
/// no node is returned, a 0x24-byte, 0x10-aligned node is allocated from TLS
/// slot 0's allocator table and initialized with the frame vtable and zeroed
/// state fields. In either path the node receives type 9 at `+0x06` and the
/// request pointer at `+0x08`, and the node pointer is returned. The allocator
/// returning null faults on the following field writes.
///
/// Original: cdecl with one stack pointer; lookup and allocator calls are
/// intercepted, with the allocator reached through its table.
lf_checker_rt::export!(cdecl, rw_0087b620(request: u32) -> u32 {
    const EXISTING_NODE_OFFSET: u32 = 0x20;
    const NODE_TYPE_OFFSET: u32 = 0x06;
    const REQUEST_OWNER_OFFSET: u32 = 0x08;
    const TLS_SLOT: usize = 0;
    const ALLOCATOR_OFFSET: u32 = 0x08;
    const ALLOCATE_VTABLE_SLOT: u32 = 0x08;
    const FRAME_NODE_VTABLE: u32 = 0x00FE83D4;
    const FRAME_NODE_TYPE: u32 = 9;
    const ALLOCATION_SIZE: u32 = 0x24;
    const ALLOCATION_ALIGNMENT: u32 = 0x10;
    const LOOKUP_EXISTING_NODE: u32 = 1;

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
                LOOKUP_EXISTING_NODE,
                u32,
                existing_node,
                FRAME_NODE_TYPE
            );
        }

        if node == 0 {
            let tls_anchor = lf_checker_rt::tls_slot(TLS_SLOT);
            let allocator = read_u32(tls_anchor.wrapping_add(ALLOCATOR_OFFSET));
            let allocator_vtable = read_u32(allocator);
            let allocate_target = read_u32(allocator_vtable.wrapping_add(ALLOCATE_VTABLE_SLOT));
            let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(allocate_target as usize);
            node = allocate(allocator, ALLOCATION_SIZE, ALLOCATION_ALIGNMENT, 0);

            write_u32(node.wrapping_add(0x04), 0x0009_0000);
            write_u32(node.wrapping_add(0x08), 0);
            write_u32(node.wrapping_add(0x0C), 0);
            write_u32(node.wrapping_add(0x10), 0);
            write_u32(node.wrapping_add(0x14), 0);
            write_u32(node.wrapping_add(0x18), 0);
            write_u32(node, lf_checker_rt::relocated(FRAME_NODE_VTABLE));
            write_u32(node.wrapping_add(0x1C), 0);
            unsafe { (node.wrapping_add(0x20) as *mut u8).write(0) };
        }

        unsafe { (node.wrapping_add(NODE_TYPE_OFFSET) as *mut u16).write_unaligned(FRAME_NODE_TYPE as u16) };
        write_u32(node.wrapping_add(REQUEST_OWNER_OFFSET), request);
        node
    }
});
