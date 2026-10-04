// original: 0x00b33bb0 sift_hole_down (proposed)

/// Sift a hole down through an array of 16-byte keyed elements, then report.
///
/// `arr` points at the array, `start` is the hole's first position and `len`
/// the element count; each element's first word is a float key. While twice
/// the hole plus 2 is below `len`, the two children there are compared: the
/// hole moves to the second child, or to the first when the first key is
/// strictly greater than the second (an unordered pair keeps the second),
/// and the child's element moves into the hole. When the walk ends exactly at
/// `len`, the last element moves into the hole. A report callee then takes
/// the array, the hole's final position, the start position, the four words
/// `w0` to `w3` and `extra`. Returns the callee's answer.
///
/// Original: 0x00b33bb0 (cdecl, eight stack words; one eight-word callee).
lf_checker_rt::export!(cdecl, rw_00b33bb0(
    arr: u32,
    start: u32,
    len: u32,
    w0: u32,
    w1: u32,
    w2: u32,
    w3: u32,
    extra: u32,
) -> u32 {
    unsafe {
        const ELEM: u32 = 16;
        const REPORT: u32 = 1;
        #[inline(always)]
        unsafe fn key(arr: u32, i: u32) -> f32 {
            unsafe {
                f32::from_bits(
                    (arr as *const u32).byte_add(i.wrapping_mul(16) as usize).read_unaligned(),
                )
            }
        }
        #[inline(always)]
        unsafe fn move_elem(arr: u32, dst: u32, src: u32) {
            unsafe {
                let d = arr.wrapping_add(dst.wrapping_mul(ELEM)) as *mut u8;
                let s = arr.wrapping_add(src.wrapping_mul(ELEM)) as *const u8;
                for off in [0usize, 8] {
                    (d.byte_add(off) as *mut u64)
                        .write_unaligned((s.byte_add(off) as *const u64).read_unaligned());
                }
            }
        }
        let start_i = start as i32;
        let len_i = len as i32;
        let mut hole = start_i;
        let mut child = start_i.wrapping_mul(2).wrapping_add(2);
        while child < len_i {
            let cu = child as u32;
            let pick = if key(arr, cu.wrapping_sub(1)) > key(arr, cu) {
                cu.wrapping_sub(1)
            } else {
                cu
            };
            move_elem(arr, hole as u32, pick);
            hole = pick as i32;
            child = (pick as i32).wrapping_mul(2).wrapping_add(2);
        }
        let mut hole_u = hole as u32;
        if child == len_i {
            move_elem(arr, hole_u, (child as u32).wrapping_sub(1));
            hole_u = (child as u32).wrapping_sub(1);
        }
        lf_checker_rt::callee_cdecl!(REPORT, u32, arr, hole_u, start, w0, w1, w2, w3, extra)
    }
});
