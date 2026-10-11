// original: 0x00876B00 crmt_frame_request_cleanup

/// Install the frame request's intermediate vtable, clear its elapsed-state
/// word, and invoke the embedded observer teardown up to three times while
/// re-reading the member at +0x0c. The embedded vtable changes before the
/// second call and reaches its final table after the third. Each helper call
/// receives the member in ECX and the embedded object at +0x04 on the stack.
/// The method has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00876b00(this: u32) -> () {
    const SUBOBJECT: u32 = 0x04;
    const OBSERVER_MEMBER: u32 = 0x0c;
    const ELAPSED_TICKS: u32 = 0x14;
    const FRAME_VTABLE: u32 = 0x00fe8224;
    const REQUEST_VTABLE: u32 = 0x00fe7fc8;
    const OBSERVER_VTABLE: u32 = 0x00fe7fb4;
    const FINAL_VTABLE: u32 = 0x00e86afc;
    const OBSERVER_TEARDOWN: u32 = 1;

    unsafe {
        let embedded = this + SUBOBJECT;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(FRAME_VTABLE));
        ((this + ELAPSED_TICKS) as *mut u32).write_unaligned(0);

        let member0 = ((this + OBSERVER_MEMBER) as *const u32).read_unaligned();
        if member0 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member0, embedded);
        }
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(REQUEST_VTABLE));

        let member1 = ((this + OBSERVER_MEMBER) as *const u32).read_unaligned();
        if member1 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member1, embedded);
        }
        let member2 = ((this + OBSERVER_MEMBER) as *const u32).read_unaligned();
        (embedded as *mut u32).write_unaligned(lf_checker_rt::relocated(OBSERVER_VTABLE));
        if member2 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member2, embedded);
        }
        (embedded as *mut u32).write_unaligned(lf_checker_rt::relocated(FINAL_VTABLE));
    }
});
