// original: 0x009d0fd0 copy_timing_triplets

/// Optionally bracket the copy with profiler callbacks, copy three adjacent 64-bit blocks from object offsets 0x2D4, 0x2DC, and 0x2E4 to the caller's destination, return that destination, and pop the one stack argument.
lf_checker_rt::export!(thiscall, rw_009d0fd0(this: u32, destination: u32) -> u32 {
    const PROFILE_BEGIN_ID: u32 = 2;
    const PROFILE_END_ID: u32 = 3;
    const PROFILE_TOKEN_VA: u32 = 0x0129588C;
    const PROFILER_ENABLED_VA: u32 = 0x0103AD58;
    const SOURCE_OFFSET: u32 = 0x2D4;
    unsafe {
        if lf_checker_rt::global::<u8>(PROFILER_ENABLED_VA).read() != 0 {
            let _ = lf_checker_rt::callee_stdcall!(PROFILE_BEGIN_ID, u32, lf_checker_rt::relocated(PROFILE_TOKEN_VA));
        }
        let source = (this as *const u8).add(SOURCE_OFFSET as usize);
        let destination = destination as *mut u8;
        let first = (source as *const u64).read_unaligned();
        let second = source.add(8).cast::<u64>().read_unaligned();
        let third = source.add(16).cast::<u64>().read_unaligned();
        destination.cast::<u64>().write_unaligned(first);
        destination.add(8).cast::<u64>().write_unaligned(second);
        destination.add(16).cast::<u64>().write_unaligned(third);
        if lf_checker_rt::global::<u8>(PROFILER_ENABLED_VA).read() != 0 {
            let _ = lf_checker_rt::callee_stdcall!(PROFILE_END_ID, u32, lf_checker_rt::relocated(PROFILE_TOKEN_VA));
        }
        destination as usize as u32
    }
});
