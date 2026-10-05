// original: 0x00AF74E0 veh_handle_teardown (proposed)

/// Tear down a handle object, releasing its three owned references.
///
/// Each of the dwords at `this + 0x6C`, `this + 0x138` and `this + 0x70`
/// that is non-null is passed by address to the releaser (callee 1) and then
/// cleared. Afterwards the three embedded members at `this + 0xD0`,
/// `this + 0xA0` and `this + 0x74` are each shut down through callee 2, the
/// last one as a tail call. Nothing is returned.
///
/// Original: 0x00AF74E0 (thiscall, no stack arguments, ends in a tail jump).
lf_checker_rt::export!(thiscall, rw_00AF74E0(this: u32) -> u32 {
    unsafe {
        const RELEASE: u32 = 1;
        const SHUT_MEMBER: u32 = 2;
        for off in [0x6Cu32, 0x138, 0x70] {
            let slot = (this + off) as *const u32;
            if slot.read_unaligned() != 0 {
                lf_checker_rt::callee_thiscall!(RELEASE, u32, slot.read_unaligned(), this + off);
                ((this + off) as *mut u32).write_unaligned(0);
            }
        }
        lf_checker_rt::callee_thiscall!(SHUT_MEMBER, u32, this + 0xD0);
        lf_checker_rt::callee_thiscall!(SHUT_MEMBER, u32, this + 0xA0);
        lf_checker_rt::callee_thiscall!(SHUT_MEMBER, u32, this + 0x74);
        0
    }
});
