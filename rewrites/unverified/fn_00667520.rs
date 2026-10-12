// original: 0x00667520 compare_emitters_by_distance

/// Compare the scalar distances stored at offset `DISTANCE` in two indirect
/// records. The cdecl arguments point to handles; each handle contains a
/// record pointer. Return 1 when the second distance is ordered-greater-than
/// or equal to the first, including equal infinities and signed zeros. Return
/// -1 for a smaller value or any unordered comparison involving a NaN.
///
/// The input layout uses 32-bit pointers as required by this original ABI:
/// `handle -> record`, with the IEEE-754 `f32` distance at byte offset 16.
lf_checker_rt::export!(cdecl, rw_00667520(first_handle: u32, second_handle: u32) -> i32 {
    unsafe {
        const RECORD_POINTER: u32 = 0;
        const DISTANCE: u32 = 0x10;

        #[inline(always)]
        unsafe fn read_pointer(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_distance(address: u32) -> f32 {
            let bits = unsafe { (address as *const u32).read_unaligned() };
            f32::from_bits(bits)
        }

        let first_record = read_pointer(first_handle + RECORD_POINTER);
        let second_record = read_pointer(second_handle + RECORD_POINTER);
        let first_distance = read_distance(first_record + DISTANCE);
        let second_distance = read_distance(second_record + DISTANCE);

        if second_distance >= first_distance { 1 } else { -1 }
    }
});
