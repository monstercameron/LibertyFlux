// original: 0x00da3bd0 copy_via_callee_scratch
/// Resolve a record through the helper and copy its four words out.
///
/// Hands a pointer to a fresh 16-byte frame area plus the two incoming
/// words to the helper, then copies the four words at the helper's
/// returned pointer into the destination. Returns the last copied word.
export!(cdecl, rw_00da3bd0(first: u32, second: u32, dest: *mut u8) -> u32 {
    unsafe {
        let mut area = [0u32; 4];
        let found = callee_cdecl!(1, u32, area.as_mut_ptr() as u32, second, first) as *const u8;
        *(dest as *mut u32) = *(found as *const u32);
        *(dest.add(4) as *mut u32) = *(found.add(4) as *const u32);
        *(dest.add(8) as *mut u32) = *(found.add(8) as *const u32);
        let last = *(found.add(12) as *const u32);
        *(dest.add(12) as *mut u32) = last;
        last
    }
});
