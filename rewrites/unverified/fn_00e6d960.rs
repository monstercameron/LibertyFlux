// original: 0x00E6D960 allocate_store_register_00e6d960

/// Allocate a region, store the returned pointer in one global slot,
/// and register the corresponding callback. The first callee receives the
/// allocation size 0x00000C00; the second receives the relocated
/// callback 0x00E72EC0. Both calls use caller-cleaned one-word arguments.
/// The stored global is at 0x01057808 and is compared as its own four-byte
/// range. The function returns the second callee's EAX value.

lf_checker_rt::export!(cdecl, rw_allocate_store_register_00e6d960() -> u32 {
    unsafe {
        const ALLOCATION_SIZE: u32 = 0x00000C00;
        const CALLBACK_VA: u32 = 0x00E72EC0;
        const RESULT_SLOT_VA: u32 = 0x01057808;
        let allocation = lf_checker_rt::callee_cdecl!(1, u32, ALLOCATION_SIZE);
        lf_checker_rt::global::<u32>(RESULT_SLOT_VA)
            .write_unaligned(allocation.wrapping_add(0x00000000));
        let callback = lf_checker_rt::relocated(CALLBACK_VA);
        lf_checker_rt::callee_cdecl!(2, u32, callback)
    }
});
