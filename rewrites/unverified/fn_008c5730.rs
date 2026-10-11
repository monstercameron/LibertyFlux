// original: 0x008C5730 fn_008c5730
/// Initialize the fixed file-memory state object and its fourteen paired
/// descriptor arrays. Each child slot is passed to the constructor stand-in;
/// the 64-entry UTF-16 scratch table is cleared as part of initialization.
lf_checker_rt::export!(thiscall, rw_008c5730(this: u32) -> u32 {
    unsafe { initialize_file_memory_state(this, false) }
});

/// Deliberately wrong version: leave the invalid-slot sentinel at its input value.
lf_checker_rt::export!(thiscall, mut_008c5730(this: u32) -> u32 {
    unsafe { initialize_file_memory_state(this, true) }
});
