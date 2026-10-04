// original: 0x00b02030 heapsort_with_mode
/// Scratch byte through the fill routine, then sort the range. The fill
/// routine takes a pointer to scratch and a zero and returns the pointer
/// (identity return, seen in its 10-byte body); the caller reads the result
/// byte through the RETURNED pointer, not the scratch register, so this
/// rewrite does the same. The contract scripts heap-pointer answers, which
/// both sides dereference for the mode byte.
export!(cdecl, rw_00b02030(a: u32, b: u32, c: u32) -> u32 {
    let mut slot: u32 = 0;
    let tmp = (&mut slot as *mut u32) as u32;
    let p: u32 = callee_cdecl!(1, u32, tmp, 0);
    let mode = unsafe { (p as *const u8).read() };
    callee_cdecl!(2, u32, a, b, c, 0, mode as u32)
});
