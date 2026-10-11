// original: 0x00DE4070 UIRowLayout::vf0

/// Call the one cdecl handler with the relocated immutable descriptor pointer and return its EAX result.
///
/// The method reads no object fields, writes no memory, and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00de4070(_this_ptr: u32) -> u32 {
    const DESCRIPTOR_FILE_VA: u32 = 0x00EFE2DC;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(DESCRIPTOR_FILE_VA))
});
