// original: 0x00c46b90 ccamfinal_set_target (proposed)
/// Store the target object pointer and cache its id word.
///
/// `arg` (a pointer or null) is stored at `this + TARGET`. When non-null,
/// the word at `arg + TARGET_ID` is copied to `this + TARGET_ID_CACHE`;
/// when null the cache is set to `NONE` (-1). Returns the cached word
/// (0 on the null path, as the original leaves it in eax).
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c46b90(this: u32, arg: u32) -> u32 {
    const TARGET: u32 = 0x1ac;
    const TARGET_ID: u32 = 0x53c;
    const TARGET_ID_CACHE: u32 = 0x1b0;
    const NONE: u32 = 0xffff_ffff;
    unsafe {
        ((this + TARGET) as *mut u32).write_unaligned(arg);
        if arg != 0 {
            let id = ((arg + TARGET_ID) as *const u32).read_unaligned();
            ((this + TARGET_ID_CACHE) as *mut u32).write_unaligned(id);
            id
        } else {
            ((this + TARGET_ID_CACHE) as *mut u32).write_unaligned(NONE);
            0
        }
    }
});
