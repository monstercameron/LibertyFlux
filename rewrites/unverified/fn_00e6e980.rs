// original: 0x00E6E980 dispatch_global_entry_if_present_00e6e980

/// Read the 16-bit count at 0x019d220e. When it is nonzero, call the
/// scripted stdcall helper with the pointer-like dword at 0x019d2208 and the
/// zero-extended count; otherwise return the count. The call site is direct, and
/// the proof includes both zero and nonzero counts.

lf_checker_rt::export!(cdecl, rw_dispatch_global_entry_if_present_00e6e980() -> u32 {
    unsafe {
        const OBJECT_POINTER_VA: u32 = 0x019D2208;
        const OBJECT_COUNT_WORD_VA: u32 = 0x019D220E;
        let count = lf_checker_rt::global::<u16>(OBJECT_COUNT_WORD_VA).read_unaligned() as u32;
        let count = count.wrapping_add(0);
        if count != 0 {
            let object = lf_checker_rt::global::<u32>(OBJECT_POINTER_VA).read_unaligned();
            lf_checker_rt::callee_stdcall!(1, u32, object, count)
        } else {
            u32::from(count)
        }
    }
});
