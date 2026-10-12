// original: 0x00D5B2F0 CLoaderDirect::process_request_queue

/// Drain the head of the eight-entry request ring while entries are ready.
/// The head and signed pending length are at `this + 0x3138` and `+0x313c`;
/// the entry pointer array starts at `+0x3114`. The readiness helper receives
/// the entry's handle at `+0x0c`. A ready entry advances the head with an
/// eight-slot wrap, decrements the pending length, and calls the completion
/// helper with the new head entry. A not-ready entry causes a ten-unit wait
/// before another poll. The retry counter wraps after one hundred waits but
/// is otherwise local to the call.
lf_checker_rt::export!(thiscall, rw_00d5b2f0(this_ptr: u32) -> () {
    unsafe {
        const REQUEST_RING: u32 = 0x3114;
        const RING_HEAD: u32 = 0x3138;
        const RING_LENGTH: u32 = 0x313c;
        const CAPACITY: u32 = 8;
        const ENTRY_READY_HANDLE: u32 = 0x0c;
        const WAIT_QUANTUM: u32 = 10;
        const RETRY_LIMIT: u32 = 1000;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let ring = this_ptr.wrapping_add(REQUEST_RING);
        let mut elapsed_wait = 0u32;
        while read_u32(this_ptr.wrapping_add(RING_LENGTH)) != 0 {
            let old_head = read_u32(this_ptr.wrapping_add(RING_HEAD));
            let next_head = old_head.wrapping_add(1);
            let next_head = if next_head == CAPACITY { 0 } else { next_head };
            let entry = read_u32(ring.wrapping_add(next_head.wrapping_mul(4)));
            let ready = lf_checker_rt::callee_cdecl!(
                1,
                u32,
                read_u32(entry.wrapping_add(ENTRY_READY_HANDLE))
            );

            if ready as u8 != 0 {
                write_u32(this_ptr.wrapping_add(RING_HEAD), next_head);
                let length = read_u32(this_ptr.wrapping_add(RING_LENGTH));
                write_u32(this_ptr.wrapping_add(RING_LENGTH), length.wrapping_sub(1));
                let completion_entry = read_u32(ring.wrapping_add(next_head.wrapping_mul(4)));
                let _ = lf_checker_rt::callee_thiscall!(3, u32, this_ptr, completion_entry);
            } else {
                let _ = lf_checker_rt::callee_cdecl!(2, u32, WAIT_QUANTUM);
                elapsed_wait = elapsed_wait.wrapping_add(WAIT_QUANTUM);
                if elapsed_wait == RETRY_LIMIT {
                    elapsed_wait = 0;
                }
            }
        }
    }
});

