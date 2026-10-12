// original: 0x00DE7010 UITimeOverview::UITimeOverview_2

/// Install the relocated time-overview vtable, invoke the clip probe, then
/// tail-forward `this` to the frame child-destruction method. The probe's
/// return is ignored; the tail callee's full 32-bit result becomes this
/// function's result. This is a zero-argument `thiscall` method.
lf_checker_rt::export!(thiscall, rw_00DE7010(this: u32) -> u32 {
    const TIME_OVERVIEW_VTABLE: u32 = 0x00EFE864;
    const CALLEE_CLIP_PROBE: u32 = 1;
    const CALLEE_DESTROY_CHILDREN: u32 = 2;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TIME_OVERVIEW_VTABLE));
    }
    let _ = lf_checker_rt::callee_thiscall!(CALLEE_CLIP_PROBE, u32, this);
    lf_checker_rt::callee_thiscall!(CALLEE_DESTROY_CHILDREN, u32, this)
});
