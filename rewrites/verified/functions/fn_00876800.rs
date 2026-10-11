// original: 0x00876800 crmt_expression_request_cleanup

/// Release the optional source at +0x18 through its virtual release slot,
/// clear that pointer and elapsed state at +0x14, then clean the embedded
/// observer at +0x04. The embedded member at +0x0c is re-read around two
/// direct teardown calls; its vtable changes before the second call and is
/// finalized afterward. Each helper call receives the member in ECX and the
/// embedded object as one stack argument. The method returns no value.
lf_checker_rt::export!(thiscall, rw_00876800(this: u32) -> () {
    const SOURCE: u32 = 0x18;
    const ELAPSED_TICKS: u32 = 0x14;
    const SUBOBJECT: u32 = 0x04;
    const OBSERVER_MEMBER: u32 = 0x0c;
    const RELEASE_SLOT: u32 = 8;
    const EXPRESSION_VTABLE: u32 = 0x00fe8204;
    const REQUEST_VTABLE: u32 = 0x00fe7fc8;
    const OBSERVER_VTABLE: u32 = 0x00fe7fb4;
    const FINAL_VTABLE: u32 = 0x00e86afc;
    const OBSERVER_TEARDOWN: u32 = 2;

    #[inline(always)]
    unsafe fn release_source(source: u32) -> u32 {
        unsafe {
            let vtable = (source as *const u32).read_unaligned();
            let target = (vtable.wrapping_add(RELEASE_SLOT) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            release(source)
        }
    }

    unsafe {
        let source = ((this + SOURCE) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(EXPRESSION_VTABLE));
        if source != 0 {
            let _ = release_source(source);
        }
        ((this + SOURCE) as *mut u32).write_unaligned(0);
        ((this + ELAPSED_TICKS) as *mut u32).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(REQUEST_VTABLE));

        let embedded = this + SUBOBJECT;
        let member = ((this + OBSERVER_MEMBER) as *const u32).read_unaligned();
        if member != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member, embedded);
        }
        let member_again = ((this + OBSERVER_MEMBER) as *const u32).read_unaligned();
        (embedded as *mut u32).write_unaligned(lf_checker_rt::relocated(OBSERVER_VTABLE));
        if member_again != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member_again, embedded);
        }
        (embedded as *mut u32).write_unaligned(lf_checker_rt::relocated(FINAL_VTABLE));
    }
});
