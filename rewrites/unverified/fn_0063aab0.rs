// original: 0x0063AAB0 rage::ptxRuleProp::construct

/// Initialize the ptxRuleProp object after its base constructor. Clear the
/// fixed state words, ask the TLS range service to allocate an optional range,
/// and store the inline-buffer pointer at offset 0x2c. The range helper's
/// return value is added to the service-provided start only when both the
/// allocation result and start are nonzero; that intermediate is then replaced
/// by the inline-buffer pointer. The vtable word is relocated for the loaded
/// image.
lf_checker_rt::export!(thiscall, rw_0063aab0(this: *mut u8) -> *mut u8 {
    unsafe {
        const BASE_CONSTRUCTOR: u32 = 1;
        const RANGE_ALLOCATOR: u32 = 2;
        const RANGE_HELPER: u32 = 3;
        const TLS_MANAGER_OFFSET: usize = 4;
        const SERVICE_CONTEXT_OFFSET: usize = 0;
        const OBJECT_VTABLE: usize = 0;
        const OBJECT_ZERO_WORD: usize = 0x18;
        const OBJECT_STATE_WORD: usize = 0x28;
        const OBJECT_RANGE: usize = 0x2c;
        const INLINE_BUFFER: usize = 0x30;
        const OBJECT_FLAG: usize = 0x31;

        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this as u32);
        let tls_block = lf_checker_rt::tls_slot(0) as usize as *const u8;
        let manager = (tls_block.add(TLS_MANAGER_OFFSET) as *const u32).read_unaligned();
        let range_slot = this.add(OBJECT_RANGE) as *mut u32;

        if manager != 0 {
            let manager_ptr = manager as usize as *const u8;
            let service = (manager_ptr.add(SERVICE_CONTEXT_OFFSET) as *const u32).read_unaligned();
            let allocation = lf_checker_rt::callee_thiscall!(RANGE_ALLOCATOR, u32, service, range_slot as u32);
            if allocation == u32::MAX {
                range_slot.write_unaligned(0);
            } else {
                let value = range_slot.read_unaligned();
                if value != 0 {
                    let adjustment = {
                lf_checker_rt::callee_thiscall!(RANGE_HELPER, u32, manager, value)
                    };
                    range_slot.write_unaligned(value.wrapping_add(adjustment));
                }
            }
        } else {
            range_slot.write_unaligned(0);
        }

        (this.add(OBJECT_VTABLE) as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00FE355C));
        (this.add(OBJECT_ZERO_WORD) as *mut u32).write_unaligned(0);
        (this.add(0x1c) as *mut u32).write_unaligned(0);
        (this.add(0x20) as *mut u32).write_unaligned(0);
        (this.add(0x24) as *mut u32).write_unaligned(0);
        (this.add(OBJECT_STATE_WORD) as *mut u32).write_unaligned(u32::MAX);
        range_slot.write_unaligned(this.add(INLINE_BUFFER) as u32);
        this.add(OBJECT_FLAG).write(0);
        this
    }
});
