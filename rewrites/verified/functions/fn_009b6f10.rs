// original: 0x009B6F10 notify_viewport_unless_ready (proposed)
/// Notify the viewport chain unless both inputs are set and it is ready.
///
/// When `a0` or `a1` has a zero low byte, or the ready flag on the object is
/// clear, runs the two-step notify chain and returns its result; otherwise
/// returns `a1` unchanged. thiscall, object in ecx.
lf_checker_rt::export!(thiscall, rw_009B6F10(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const READY_FLAG: u32 = 0x532;
        let go = a0 as u8 == 0 || a1 as u8 == 0 || ((this + READY_FLAG) as *const u8).read_unaligned() == 0;
        if go {
            let r: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, a0, a1);
            lf_checker_rt::callee_thiscall!(2, u32, this, r)
        } else {
            a1
        }
    }
});
