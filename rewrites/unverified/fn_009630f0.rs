// original: 0x009630f0 name_copy_60
/// Copy a name string into this record's second slot (offset 0x60).
///
/// Same shape as `rw_009630c0` with a 0x3F-byte limit and terminator at
/// +0x9F.
export!(thiscall, rw_009630f0(this_ptr: u32, src: u32) -> u32 {
    unsafe {
        if src == 0 {
            return 0;
        }
        if *(src as *const u8) == 0 {
            return src;
        }
        let ans = callee_cdecl!(1, u32, this_ptr.wrapping_add(0x60), src, 0x3Fu32);
        *((this_ptr as *mut u8).add(0x9F)) = 0;
        ans
    }
});
