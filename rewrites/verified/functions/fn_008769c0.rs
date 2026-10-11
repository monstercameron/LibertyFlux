// original: 0x008769C0 crmt_extrapolate_request_reset

/// Clear elapsed ticks, the two extrapolation words at +0x1c/+0x20, and the
/// embedded-source state in order. Install the base request vtable, invoke
/// the embedded observer teardown for a non-null member at +0x0c, change its
/// vtable, re-read the member and invoke the helper again if still present,
/// then install the observer's final vtable. The helper receives the member
/// in ECX and the embedded object at +0x04 as one stack argument. No return
/// value is used.
lf_checker_rt::export!(thiscall, rw_008769c0(this: u32) -> () {
    const SUBOBJECT: u32 = 0x04;
    const MEMBER: u32 = 0x0c;
    const ELAPSED_TICKS: u32 = 0x14;
    const EXTRAPOLATION_START: u32 = 0x1c;
    const EXTRAPOLATION_END: u32 = 0x20;
    const REQUEST_VTABLE: u32 = 0x00fe7fc8;
    const OBSERVER_VTABLE: u32 = 0x00fe7fb4;
    const FINAL_VTABLE: u32 = 0x00e86afc;
    const OBSERVER_TEARDOWN: u32 = 1;

    unsafe {
        ((this + EXTRAPOLATION_START) as *mut u32).write_unaligned(0);
        ((this + EXTRAPOLATION_END) as *mut u32).write_unaligned(0);
        ((this + ELAPSED_TICKS) as *mut u32).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(REQUEST_VTABLE));

        let subobject = this + SUBOBJECT;
        let member = ((this + MEMBER) as *const u32).read_unaligned();
        if member != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member, subobject);
        }
        let member_again = ((this + MEMBER) as *const u32).read_unaligned();
        (subobject as *mut u32).write_unaligned(lf_checker_rt::relocated(OBSERVER_VTABLE));
        if member_again != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member_again, subobject);
        }
        (subobject as *mut u32).write_unaligned(lf_checker_rt::relocated(FINAL_VTABLE));
    }
});
