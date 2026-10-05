// original: 0x00a4ef20 sort_heap_sort (proposed)

/// Heap sort over [first, last): make-heap, extend, then pop down.
///
/// Builds a heap through the make-heap callee, then grows it toward `lim`
/// one slot at a time: any slot whose key the first slot's key strictly
/// exceeds is moved to the front and the adjust-heap callee repairs the
/// heap. Finally the range is popped from the back through the pop-heap
/// callee while more than one element remains. `extra` threads through.
/// Note: the original spills `extra`'s low byte into its own dead second
/// argument slot and passes the spliced word on; the value is reproduced
/// here but the stack write itself is not (see the contract's checks).
/// Cdecl, five stack words (fourth unused), three callees, no result.
lf_checker_rt::export!(cdecl, rw_00a4ef20(first: u32, last: u32, lim: u32, _u: u32, extra: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x80;
        const MAKE: u32 = 1;
        const ADJUST: u32 = 2;
        const POP: u32 = 3;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe fn key(elem: u32) -> f32 {
            unsafe { f32::from_bits(rd(elem.wrapping_add(KEY_OFF))) }
        }
        lf_checker_rt::callee_cdecl!(MAKE, u32, first, last, extra);
        let mut cur = last;
        if cur < lim {
            let idx = ((last.wrapping_sub(first)) as i32) >> 2;
            loop {
                let a = rd(first);
                let c = rd(cur);
                if key(a) > key(c) {
                    wr(cur, a);
                    lf_checker_rt::callee_cdecl!(ADJUST, u32, first, 0, idx as u32, c, extra);
                }
                cur = cur.wrapping_add(4);
                if !(cur < lim) {
                    break;
                }
            }
        }
        let mut span = last.wrapping_sub(first);
        let mut tail = last;
        let spliced = (last & 0xffff_ff00) | (extra & 0xff);
        if ((span & !3) as i32) > 4 {
            loop {
                lf_checker_rt::callee_cdecl!(POP, u32, first, tail, spliced);
                span = span.wrapping_sub(4);
                tail = tail.wrapping_sub(4);
                if ((span & !3) as i32) <= 4 {
                    break;
                }
            }
        }
        0
    }
});
