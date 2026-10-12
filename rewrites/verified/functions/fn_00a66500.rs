// original: 0x00A66500 insertion_sort_frag_pairs_unguarded

/// Visit every eight-byte fragment pair in the half-open `[begin, end)` range
/// and pass it to the insertion helper. The third cdecl argument is ignored;
/// the fourth is forwarded as the helper context. Each cdecl helper call
/// receives the cursor, the copied pair's two words, and context as stack
/// arguments. The local pair's frame address is not compared; its contents
/// are observed as call arguments. The outer routine is cdecl and has no
/// meaningful return value.
lf_checker_rt::export!(cdecl, rw_00a66500(begin: u32, end: u32, _reserved: u32, context: u32) -> u32 {
    unsafe {
        const PAIR_STRIDE: u32 = 8;
        const PAIR_SECOND_WORD: u32 = 4;
        const INSERT_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let mut cursor = begin;
        while cursor != end {
            let mut pivot_pair = [
                read_u32(cursor),
                read_u32(cursor.wrapping_add(PAIR_SECOND_WORD)),
            ];
            let _ignored_result = lf_checker_rt::callee_cdecl!(
                INSERT_CALLEE,
                u32,
                cursor,
                pivot_pair[0],
                pivot_pair[1],
                context,
            );
            cursor = cursor.wrapping_add(PAIR_STRIDE);
        }
        0
    }
});
