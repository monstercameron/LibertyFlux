// original: 0x00b33a50 peds_sift_down_key0 (proposed)

/// Sift-down step of a min-heap removal over 28-byte records, then hand off.
///
/// `array` points at `count` records of 28 bytes; `index` is the hole left by
/// the removed head. While the hole has two children (indices `2*i+1` and
/// `2*i+2`), the child with the smaller float key (dword at record+0x00)
/// is moved into the hole; an unordered NaN comparison keeps the left child, like
/// the original's `comiss` followed by `jbe`. With a single child left
/// (`2*i+2 == count`) that child is moved in. The callee then receives the
/// array, the final hole, the initial index, the seven words `w14..w2c` and
/// `w30` verbatim, and its answer is returned.
///
/// Both children are staged through locals in the original's load order, and
/// records move in its chunk order (three 8-byte chunks, then the trailing
/// word), so fault behaviour matches access for access.
///
/// Original: 0x00b33a50 (cdecl, eleven stack words).
lf_checker_rt::export!(cdecl, rw_00b33a50(array: u32, index: u32, count: u32, w14: u32, w18: u32, w1c: u32, w20: u32, w24: u32, w28: u32, w2c: u32, w30: u32) -> u32 {
    unsafe {
        const REC_LEN: u32 = 28;
        const KEY_OFF: u32 = 0x00;
        const HOLE_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Move one 28-byte record, in the original's chunk order.
        #[inline(always)]
        unsafe fn move_rec(dst: u32, src: u32) {
            unsafe {
                wr64(dst, rd64(src));
                wr64(dst + 8, rd64(src + 8));
                wr64(dst + 0x10, rd64(src + 0x10));
                wr32(dst + 0x18, rd32(src + 0x18));
            }
        }

        let mut hole = index;
        let mut child = index.wrapping_mul(2).wrapping_add(2);
        if (child as i32) < (count as i32) {
            loop {
                let left = array.wrapping_add(child.wrapping_sub(1).wrapping_mul(REC_LEN));
                let right = array.wrapping_add(child.wrapping_mul(REC_LEN));
                let _stage0 = rd64(left);
                let _stage1 = rd32(left + 0x10);
                let _stage2 = rd64(left + 8);
                let _stage3 = rd64(right);
                let _stage4 = rd64(right + 8);
                let _stage5 = rd32(right + 0x10);
                let left_key = f32::from_bits(rd32(left + KEY_OFF));
                let right_key = f32::from_bits(rd32(right + KEY_OFF));
                if left_key > right_key {
                    child = child.wrapping_sub(1);
                }
                let dst = array.wrapping_add(hole.wrapping_mul(REC_LEN));
                let src = array.wrapping_add(child.wrapping_mul(REC_LEN));
                move_rec(dst, src);
                hole = child;
                child = child.wrapping_mul(2).wrapping_add(2);
                if !((child as i32) < (count as i32)) {
                    break;
                }
            }
        }
        if child == count {
            let last = array.wrapping_add(child.wrapping_sub(1).wrapping_mul(REC_LEN));
            let dst = array.wrapping_add(hole.wrapping_mul(REC_LEN));
            move_rec(dst, last);
            hole = child.wrapping_sub(1);
        }
        lf_checker_rt::callee_cdecl!(HOLE_CALLEE, u32, array, hole, index, w14, w18, w1c, w20, w24, w28, w2c, w30)
    }
});
