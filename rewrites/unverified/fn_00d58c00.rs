// original: 0x00d58c00 ccam_view_find_child_metric
/// Return the head child's distance metric, or +0.0 when it is not of kind
/// `WANT_KIND`.
///
/// Loads the head child at `[this+0x124]` (never null on entry) and calls
/// the kind query (the virtual slot at `+0x28`, intercepted callee 1). When
/// the answer equals `WANT_KIND` (0xe) the function tail-jumps to the metric
/// routine (intercepted tail callee 2, returning an `f32` in `st0`) and its
/// result is the result; otherwise the function returns +0.0.
///
/// Original: thiscall, no stack arguments, returns an `f32` in `st0`; the
/// metric path ends in a tail jump.
lf_checker_rt::export!(thiscall, rw_00d58c00 (this: u32) -> f32 {
    unsafe {
        const HEAD: u32 = 0x124;
        const KIND_SLOT: u32 = 0x28;
        const WANT_KIND: u32 = 0xe;
        const METRIC_TAIL: u32 = 2;
        let child = ((this + HEAD) as *const u32).read_unaligned();
        let vtable = (child as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable + KIND_SLOT) as *const u32).read_unaligned() as usize);
        if kind_of(child) == WANT_KIND {
            lf_checker_rt::callee_thiscall!(METRIC_TAIL, f32, child)
        } else {
            0.0
        }
    }
});
