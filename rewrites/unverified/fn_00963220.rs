// original: 0x00963220 name_copy_40
/// Copy a name string into this record's middle slot (offset 0x40).
///
/// Same shape as `rw_009630c0` with a 0x1F-byte limit and terminator at
/// +0x5F.
export!(thiscall, rw_00963220(this_ptr: u32, src: u32) -> u32 {
    unsafe {
        if src == 0 {
            return 0;
        }
        if *(src as *const u8) == 0 {
            return src;
        }
        let ans = callee_cdecl!(1, u32, this_ptr.wrapping_add(0x40), src, 0x1Fu32);
        *((this_ptr as *mut u8).add(0x5F)) = 0;
        ans
    }
});
