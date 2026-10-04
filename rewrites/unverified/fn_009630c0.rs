// original: 0x009630c0 name_copy_20
/// Copy a name string into this record's first slot (offset 0x20).
///
/// When the source pointer is non-null and points at a non-empty string,
/// copies up to 0x1F bytes through the copy helper (stubbed by the checker)
/// and terminates the slot at +0x3F. Returns the helper's answer on the
/// copy path, 0 for a null source, the source itself for an empty string.
export!(thiscall, rw_009630c0(this_ptr: u32, src: u32) -> u32 {
    unsafe {
        if src == 0 {
            return 0;
        }
        if *(src as *const u8) == 0 {
            return src;
        }
        let ans = callee_cdecl!(1, u32, this_ptr.wrapping_add(0x20), src, 0x1Fu32);
        *((this_ptr as *mut u8).add(0x3F)) = 0;
        ans
    }
});
