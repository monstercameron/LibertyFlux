// original: 0x00DB6E90 UIFontString::vf2

/// Scalar deleting destructor: run the destructor (thiscall, no stack args),
/// then free the object through the allocator (cdecl, one word) when bit 0
/// of the flag is set.
///
/// `thiscall`, one stack word. Returns the object pointer. The allocator
/// lives in the encrypted first megabyte, but its call site is patched like
/// any other, so the proof never executes it.
lf_checker_rt::export!(thiscall, rw_00db6e90(this_ptr: u32, flag: u32) -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, this_ptr);
    if (flag & 1) != 0 {
        lf_checker_rt::callee_cdecl!(2, u32, this_ptr);
    }
    this_ptr
});
