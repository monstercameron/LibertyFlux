// original: 0x008765A0 crmt_blend_request_teardown

/// Tear down the blend request in order: release the optional children at
/// +0x28 and +0x2c through vtable slot +0x08, clear both child slots and the
/// timing words at +0x18 and +0x14, install the base request vtable, invoke
/// the embedded observer teardown twice when its member at +0x0c remains
/// non-null, then install the observer's final vtable. The direct teardown
/// helper receives the member in ECX and the embedded object at +0x04 as its
/// one stack argument. The thiscall method has no arguments or meaningful
/// return value.
lf_checker_rt::export!(thiscall, rw_008765a0(this: u32) -> () {
    const CHILD_A: u32 = 0x28;
    const CHILD_B: u32 = 0x2c;
    const EMBEDDED: u32 = 0x04;
    const EMBEDDED_MEMBER: u32 = 0x0c;
    const ELAPSED_TICKS: u32 = 0x14;
    const START_WEIGHT: u32 = 0x18;
    const RELEASE_SLOT: u32 = 8;
    const INTERMEDIATE_VTABLE: u32 = 0x00fe81f4;
    const REQUEST_VTABLE: u32 = 0x00fe7fc8;
    const OBSERVER_VTABLE: u32 = 0x00fe7fb4;
    const FINAL_VTABLE: u32 = 0x00e86afc;
    const OBSERVER_TEARDOWN: u32 = 2;

    #[inline(always)]
    unsafe fn release_child(child: u32) -> u32 {
        unsafe {
            let vtable = (child as *const u32).read_unaligned();
            let target = (vtable.wrapping_add(RELEASE_SLOT) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            release(child)
        }
    }

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(INTERMEDIATE_VTABLE));
        let child_a = ((this + CHILD_A) as *const u32).read_unaligned();
        if child_a != 0 {
            let _ = release_child(child_a);
        }

        let child_b = ((this + CHILD_B) as *const u32).read_unaligned();
        ((this + CHILD_A) as *mut u32).write_unaligned(0);
        if child_b != 0 {
            let _ = release_child(child_b);
        }
        ((this + CHILD_B) as *mut u32).write_unaligned(0);
        ((this + START_WEIGHT) as *mut u32).write_unaligned(0);
        ((this + ELAPSED_TICKS) as *mut u32).write_unaligned(0);

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(REQUEST_VTABLE));
        let embedded = this + EMBEDDED;
        let member = ((this + EMBEDDED_MEMBER) as *const u32).read_unaligned();
        if member != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member, embedded);
        }

        let member_again = ((this + EMBEDDED_MEMBER) as *const u32).read_unaligned();
        (embedded as *mut u32).write_unaligned(lf_checker_rt::relocated(OBSERVER_VTABLE));
        if member_again != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member_again, embedded);
        }
        (embedded as *mut u32).write_unaligned(lf_checker_rt::relocated(FINAL_VTABLE));
    }
});
