// original: 0x009747d0 aud_occlusion_group_ctor (proposed)

/// Construct the occlusion-group base: stamp the base vtable, run the member
/// cleanup callee, then stamp the derived vtable. Both vtable addresses are
/// relocated image addresses, never literals.
/// Original: 0x009747D0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_009747d0(this: u32) -> u32 {
    unsafe {
        const VT_BASE: u32 = 0xE8BAB8;
        const VT_DERIVED: u32 = 0xE7C88C;
        const MEMBER_CTOR: u32 = 1;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT_BASE));
        lf_checker_rt::callee_thiscall!(MEMBER_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT_DERIVED));
        0
    }
});
