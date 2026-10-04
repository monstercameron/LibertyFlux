// original: 0x009B7640 NativeImpl_GET_VIEWPORT_POS_AND_SIZE
/// Read the viewport position and size into two out-words pairs.
///
/// Resolves the viewport entry for `a0`, then reads word 0 and word 1 of the
/// unwrapped entry into `out1` and words 2 and 3 into `out2`. Returns the
/// last word read. stdcall, three arguments.
lf_checker_rt::export!(stdcall, rw_009B7640(a0: u32, out1: u32, out2: u32) -> u32 {
    unsafe {
        const ENTRY_INNER: u32 = 0x10;
        let entry: u32 = lf_checker_rt::callee_stdcall!(1, u32, a0);
        let obj = entry.wrapping_add(ENTRY_INNER);
        let p0: u32 = lf_checker_rt::callee_thiscall!(2, u32, obj);
        (out1 as *mut u32).write_unaligned((p0 as *const u32).read_unaligned());
        let p1: u32 = lf_checker_rt::callee_thiscall!(3, u32, obj);
        ((out1 + 4) as *mut u32).write_unaligned(((p1 + 4) as *const u32).read_unaligned());
        let p2: u32 = lf_checker_rt::callee_thiscall!(4, u32, obj);
        (out2 as *mut u32).write_unaligned(((p2 + 8) as *const u32).read_unaligned());
        let p3: u32 = lf_checker_rt::callee_thiscall!(5, u32, obj);
        let last = ((p3 + 12) as *const u32).read_unaligned();
        ((out2 + 4) as *mut u32).write_unaligned(last);
        last
    }
});
