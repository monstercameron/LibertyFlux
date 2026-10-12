// original: 0x00D5AEF0 CLoaderDirect::cancel_stream_request

/// Search the pending request ring for a matching 16-bit request kind. The
/// ring starts at `this + 0x3114`; its head and length are at `+0x3138` and
/// `+0x313c`, with eight request pointers. The search begins at the slot after
/// the head and uses signed length comparisons. A ready matching request is
/// released, reported to the queue helpers, removed from the pending count,
/// and cleared; a matching request that is not ready is marked state 3. The
/// one request argument is read as a 16-bit kind. Calls are made through the
/// checker runtime so the proof observes their order and arguments.
lf_checker_rt::export!(thiscall, rw_00d5aef0(this_ptr: u32, request_kind: u32) -> u32 {
    unsafe {
        const REQUEST_RING: u32 = 0x3114;
        const RING_HEAD: u32 = 0x3138;
        const RING_LENGTH: u32 = 0x313c;
        const PENDING_COUNT: u32 = 0x04;
        const RING_CAPACITY_MASK: u32 = 7;
        const ENTRY_STATE: u32 = 0x00;
        const ENTRY_KIND: u32 = 0x08;
        const ENTRY_READY_HANDLE: u32 = 0x0c;
        const ENTRY_RESOURCE_HANDLE: u32 = 0x10;
        const ENTRY_CLEANUP_FIELD: u32 = 0x14;
        const NOT_READY_STATE: u32 = 3;
        const EMPTY_KIND: u16 = u16::MAX;
        const GLOBAL_REQUEST_SERVICE: u32 = 0x0103_e8d0;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        let ring_length = read_u32(this_ptr.wrapping_add(RING_LENGTH));
        if (ring_length as i32) <= 0 {
            return this_ptr;
        }

        let ring = this_ptr.wrapping_add(REQUEST_RING);
        let head = read_u32(this_ptr.wrapping_add(RING_HEAD));
        let mut search_index = 0u32;
        while (search_index as i32) < (ring_length as i32) {
            let raw_slot = head.wrapping_add(1).wrapping_add(search_index);
            let slot = raw_slot & RING_CAPACITY_MASK;
            let entry = read_u32(ring.wrapping_add(slot.wrapping_mul(4)));

            if read_u16(entry.wrapping_add(ENTRY_KIND)) == request_kind as u16 {
                let ready = lf_checker_rt::callee_cdecl!(
                    1,
                    u32,
                    read_u32(entry.wrapping_add(ENTRY_READY_HANDLE))
                );
                if ready as u8 == 0 {
                    write_u32(entry.wrapping_add(ENTRY_STATE), NOT_READY_STATE);
                    return ready;
                }

                let resource = read_u32(entry.wrapping_add(ENTRY_RESOURCE_HANDLE));
                let _ = lf_checker_rt::callee_cdecl!(2, u32, resource);
                let kind = u32::from(read_u16(entry.wrapping_add(ENTRY_KIND)));
                let service = lf_checker_rt::relocated(GLOBAL_REQUEST_SERVICE);
                let _ = lf_checker_rt::callee_thiscall!(3, u32, service, kind);
                let _ = lf_checker_rt::callee_thiscall!(4, u32, service, kind);
                let _ = lf_checker_rt::callee_thiscall!(5, u32, this_ptr, entry);
                let _ = lf_checker_rt::callee_thiscall!(6, u32, ring, search_index);

                let pending_count = read_u32(this_ptr.wrapping_add(PENDING_COUNT));
                write_u32(
                    this_ptr.wrapping_add(PENDING_COUNT),
                    pending_count.wrapping_sub(1),
                );
                write_u32(entry.wrapping_add(ENTRY_STATE), 0);
                write_u32(entry.wrapping_add(ENTRY_CLEANUP_FIELD), 0);
                write_u32(entry.wrapping_add(ENTRY_RESOURCE_HANDLE), u32::MAX);
                write_u32(entry.wrapping_add(4), 0);
                write_u16(entry.wrapping_add(ENTRY_KIND), EMPTY_KIND);
                return u32::from(EMPTY_KIND);
            }
            search_index = search_index.wrapping_add(1);
        }

        head.wrapping_add(1).wrapping_add(ring_length)
    }
});

