// original: 0x00683DA0 tls_allocator_loop_process_samples

/// Allocate a word buffer through the current thread's slot-zero service, then
/// process up to `count` existing entries. Negative and zero counts return the
/// allocated buffer without walking it. A zero buffer, missing service item,
/// or empty entry is skipped safely; a lookup result of `u32::MAX` clears the
/// entry, while other results are accumulated and a nonzero total triggers the
/// release callback. The allocator and callbacks are indirect or direct
/// callees in the original ABI and are intercepted by the checker contract.
/// The function is thiscall with one signed count argument and returns the
/// allocated buffer pointer.
lf_checker_rt::export!(thiscall, rw_00683da0(this: u32, count: u32) -> u32 {
    unsafe {
        const TLS_SERVICE_SLOT: usize = 0;
        const SERVICE_RELEASE: u32 = 0x04;
        const SERVICE_ALLOCATOR: u32 = 0x08;
        const ALLOCATOR_VTABLE_SLOT: u32 = 0x08;
        const BUFFER_ELEMENT_SIZE: u32 = 4;
        const ALLOCATION_KIND: u32 = 0x10;

        let provider = lf_checker_rt::tls_slot(TLS_SERVICE_SLOT);
        let allocator = crate::read_word(provider.wrapping_add(SERVICE_ALLOCATOR));
        let allocator_vtable = crate::read_word(allocator);
        let allocate_address = crate::read_word(allocator_vtable.wrapping_add(ALLOCATOR_VTABLE_SLOT));
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address as usize);
        let buffer = allocate(
            allocator,
            count.wrapping_mul(BUFFER_ELEMENT_SIZE),
            ALLOCATION_KIND,
            0,
        );

        if (count as i32) > 0 && buffer != 0 {
            let release_service = crate::read_word(provider.wrapping_add(SERVICE_RELEASE));
            let lookup_service = crate::read_word(release_service);
            let mut index = 0i32;
            while index < count as i32 {
                let entry = buffer.wrapping_add((index as u32).wrapping_mul(BUFFER_ELEMENT_SIZE));
                if release_service == 0 {
                    crate::write_word(entry, 0);
                } else {
                    let prior_count = crate::read_word(entry);
                    if prior_count == 0 {
                        crate::write_word(entry, 0);
                    } else {
                        let lookup = lf_checker_rt::callee_thiscall!(
                            2,
                            u32,
                            lookup_service,
                            entry,
                        );
                        if lookup == u32::MAX {
                            crate::write_word(entry, 0);
                        } else {
                            let delta = lf_checker_rt::callee_thiscall!(
                                3,
                                u32,
                                release_service,
                                prior_count,
                            );
                            let total = prior_count.wrapping_add(delta);
                            crate::write_word(entry, total);
                            if total != 0 {
                                let _ = lf_checker_rt::callee_thiscall!(
                                    4,
                                    u32,
                                    total,
                                    release_service,
                                );
                            }
                        }
                    }
                }
                index += 1;
            }
        }

        let _ = this;
        buffer
    }
});
