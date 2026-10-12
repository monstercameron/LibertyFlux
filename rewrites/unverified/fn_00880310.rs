// original: 0x00880310 allocate_utf16_buffer

/// Allocate a zero-terminated UTF-16 buffer through the allocator stored in
/// TLS slot zero. A zero unit count returns null. Positive counts request
/// twice the unit count in bytes, 16-byte alignment, and a zero fill value.
/// Counts above the unsigned 31-bit limit follow a fatal reporter path and
/// reach `int3`; that faulting path is excluded from the proof.
lf_checker_rt::export!(thiscall, rw_00880310(_this: u32, unit_count: u32, _unused: u32) -> u32 {
    unsafe {
        const ALLOCATOR_OFFSET: u32 = 0x08;
        const VTABLE_SLOT_OFFSET: u32 = 0x08;
        const MAX_UNITS: u32 = 0x7FFF_FFFF;
        const ALLOCATOR_CALLEE: u32 = 1;
        const REPORT_CALLEE: u32 = 2;
        const FATAL_CALLEE: u32 = 3;
        const ERROR_MESSAGE_VA: u32 = 0x00FC_A09C;

        if unit_count > MAX_UNITS {
            let message = lf_checker_rt::relocated(ERROR_MESSAGE_VA);
            let _ = lf_checker_rt::callee_cdecl!(REPORT_CALLEE, u32, message);
            let _ = lf_checker_rt::callee_stdcall!(FATAL_CALLEE, u32, 1);
            std::process::abort();
        }
        if unit_count == 0 {
            return 0;
        }

        let context = lf_checker_rt::tls_slot(0);
        let allocator = (context.wrapping_add(ALLOCATOR_OFFSET) as *const u32).read_unaligned();
        let vtable = (allocator as *const u32).read_unaligned();
        let target = (vtable.wrapping_add(VTABLE_SLOT_OFFSET) as *const u32).read_unaligned();
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        allocate(allocator, unit_count.wrapping_mul(2), 0x10, 0)
    }
});
