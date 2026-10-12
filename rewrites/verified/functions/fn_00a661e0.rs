// original: 0x00A661E0 make_heap_frag_pairs

/// Build a max heap over an array of eight-byte fragment pairs. `begin` and
/// `end` are byte pointers delimiting the half-open array; the signed element
/// count is `(end - begin) / 8`. Arrays shorter than two pairs return without
/// calls. Otherwise the routine visits parents in descending order, starting
/// at `(count - 2) / 2`, copies each pair to an eight-byte stack temporary,
/// and calls the cdecl sift-down helper with six stack words: base, parent
/// index, count, the copied pair, and caller context. The caller cleans the
/// stack after each call. The pair values are observed as call arguments; its
/// frame address is intentionally not part of the call comparison. The outer function is
/// cdecl with three stack arguments and no meaningful return value.
lf_checker_rt::export!(cdecl, rw_00a661e0(begin: u32, end: u32, context: u32) -> u32 {
    unsafe {
        const PAIR_STRIDE: u32 = 8;
        const PAIR_SECOND_WORD: u32 = 4;
        const SIFT_DOWN_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let signed_count = (end.wrapping_sub(begin) as i32) >> 3;
        if signed_count < 2 {
            return 0;
        }

        let pair_count = signed_count as u32;
        let mut parent = (pair_count - 2) / 2;
        loop {
            let pair_address = begin.wrapping_add(parent.wrapping_mul(PAIR_STRIDE));
            let mut pivot_pair = [
                read_u32(pair_address),
                read_u32(pair_address.wrapping_add(PAIR_SECOND_WORD)),
            ];
            let _ignored_result = lf_checker_rt::callee_cdecl!(
                SIFT_DOWN_CALLEE,
                u32,
                begin,
                parent,
                pair_count,
                pivot_pair[0],
                pivot_pair[1],
                context,
            );

            if parent == 0 {
                break;
            }
            parent -= 1;
        }
        0
    }
});
