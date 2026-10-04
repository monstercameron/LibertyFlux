// original: 0x00b79180 clear_arg1_field10
/// Clear the word at offset `0x10` of the second argument.
///
/// Takes two pointers, ignores the first, writes zero to `arg1 + 0x10`,
/// and returns `arg1`.
export!(cdecl, rw_00b79180(_unused: u32, ptr: u32) -> u32 {
    unsafe {
        *((ptr + 0x10) as *mut u32) = 0;
    }
    ptr
});
