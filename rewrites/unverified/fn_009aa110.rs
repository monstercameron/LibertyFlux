// original: 0x009aa110 conv_first_or_null
/// Return the first word of the entry list for `key`, if any is live.
///
/// Asks the sub-table lookup (stubbed, stdcall/2) for the entry list of
/// `key`, with the live count delivered through a stack out-byte. When
/// the count is zero there is no entry and null is returned; otherwise
/// the first word of the list is returned. Stdcall, one stack word.
///
/// The checker cannot drive the live path: its stubs write whole words,
/// and a word store through this out-pointer would clobber the return
/// address one byte above it, so the live branch is never taken.
export!(stdcall, rw_009AA110(key: u32) -> u32 {
    unsafe {
        let mut count: u8 = 0;
        let list: u32 = callee_stdcall!(1, u32, key, &mut count as *mut u8 as u32);
        if count == 0 {
            return 0;
        }
        (list as *const u32).read_unaligned()
    }
});
