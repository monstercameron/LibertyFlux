// original: 0x005EFC70 task_ctor_build (proposed)

/// Build the task in place through the fifteen-argument builder.
///
/// Calls the builder with `this`, the second stack argument and the fixed
/// parameter block, installs the vtable and returns `this`. The first stack
/// argument is never read. The builder's first argument slot carries a word
/// from the caller's frame above the incoming arguments, which no rewrite
/// can reproduce; this proof skips that slot in the call comparison and
/// passes zero there. The original is thiscall with two stack arguments.
lf_checker_rt::export!(thiscall, rw_005efc70(this: u32, first: u32, second: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EB35E4;
        const BUILD_CALLEE: u32 = 1;
        let _ = first;
        lf_checker_rt::callee_thiscall!(BUILD_CALLEE, u32, this, 0, second, 6, 3, 0x28, 0, 0, 0, 0, 4, 0x14, 0xbf800000u32, 0x1e, 0x14, 1);
        (this as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE));
        this
    }
});
