// original: 0x00A66910 heap_pop_frag_pairs

/// Remove the first fragment pair by saving the root pair at the final slot,
/// then request a sift-down of the replacement pair over the remaining range.
/// `begin` and `end` delimit eight-byte pairs; this routine reads the root and
/// final pair before writing, then calls the helper with the array, root index
/// zero, the signed bound `(pair_count - 2)`, the saved pair words, and context.
/// The helper call is cdecl and its return value is ignored. The function has
/// three cdecl stack arguments and no meaningful return value.
lf_checker_rt::export!(cdecl, rw_00a66910(begin: u32, end: u32, context: u32) -> u32 {
    const PAIR_STRIDE: u32 = 8;
    const SECOND_WORD: u32 = 4;
    const SIFT_DOWN_CALLEE: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let tail_pair = end.wrapping_sub(PAIR_STRIDE);
        let root_pair = [read_u32(begin), read_u32(begin.wrapping_add(SECOND_WORD))];
        let replacement_pair = [read_u32(tail_pair), read_u32(tail_pair.wrapping_add(SECOND_WORD))];
        write_u32(tail_pair, root_pair[0]);
        write_u32(tail_pair.wrapping_add(SECOND_WORD), root_pair[1]);

        let signed_pair_count = (end.wrapping_sub(begin) as i32) >> 3;
        let sift_bound = signed_pair_count.wrapping_sub(2) as u32;
        let _ignored_result = lf_checker_rt::callee_cdecl!(
            SIFT_DOWN_CALLEE,
            u32,
            begin,
            0,
            sift_bound,
            replacement_pair[0],
            replacement_pair[1],
            context,
        );
        0
    }
});
