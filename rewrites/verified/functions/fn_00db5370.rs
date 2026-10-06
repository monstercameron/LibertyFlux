// original: 0x00DB5370 cursor_tree_count_matches

/// Count consecutive tree entries matching one key.
///
/// `this` is the tree and `key_ptr` points at the search key. The lookup
/// helper (scripted) finds the first node at or above the key; when it
/// reports a node and the search key is not below the reported candidate
/// key (UNSIGNED `jb`), iteration starts there, otherwise the count is 0.
/// Each iteration calls the stepping helper (scripted, one scripted answer
/// object per call in order), counts the call, and continues while the
/// reported node is non-null and its key still equals the search key
/// (plain equality). The stepping helper answers a pointer to its result
/// object; the 8-byte register copy of the result in the original is a
/// plain two-word move.
///
/// Note: the bound check's signedness is unobservable — taken or not, a
/// mismatching key reaches a zero count with no stepping call on either
/// path — so the shipped wrong version inverts the loop-continue test
/// instead. One unreachable block of the original (a second null test
/// after the loop check already excluded null) is omitted.
///
/// Original: 0x00DB5370 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00db5370(this: u32, key_ptr: u32) -> u32 {
    unsafe {
        const FIND: u32 = 1;
        const STEP: u32 = 2;

        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let search = rd(key_ptr);
        let mut first = [0u32; 3];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            FIND,
            u32,
            this,
            &mut first as *mut u32 as u32,
            key_ptr,
            0
        );
        let (mut cur_key, mut cur_node) = (first[0], first[1]);
        if cur_node == 0 || search < cur_key {
            cur_key = 0;
            cur_node = 0;
        }
        let mut count = 0u32;
        loop {
            if cur_node == 0 {
                break;
            }
            if search != cur_key {
                break;
            }
            let mut next = [0u32; 3];
            let answer: u32 = lf_checker_rt::callee_thiscall!(
                STEP,
                u32,
                this,
                &mut next as *mut u32 as u32,
                cur_node
            );
            let (step_key, step_node) = (rd(answer), rd(answer + 4));
            count = count.wrapping_add(1);
            if step_node == 0 {
                break;
            }
            cur_key = step_key;
            cur_node = step_node;
        }
        count
    }
});
