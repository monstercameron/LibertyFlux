// original: 0x00ADD110 CRenderPhaseSetDefaultRenderState::ctor

/// Construct a default render-state phase at `this`. First delegates the base
/// phase setup with a zero flag, then installs the class table and sets the
/// render-state phase identifier to 3. Returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_00add110(this: u32) -> u32 {
    const BASE_PHASE_CONSTRUCTOR: u32 = 1;
    const BASE_CONSTRUCTOR_FLAG: u32 = 0;
    const VTABLE_ADDRESS: u32 = 0x00EA6EF4;
    const PHASE_IDENTIFIER_OFFSET: u32 = 0x08F4;
    const PHASE_IDENTIFIER: u32 = 3;

    unsafe {
        let _base_result = lf_checker_rt::callee_thiscall!(
            BASE_PHASE_CONSTRUCTOR,
            u32,
            this,
            BASE_CONSTRUCTOR_FLAG,
        );
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_ADDRESS));
        (this.wrapping_add(PHASE_IDENTIFIER_OFFSET) as *mut u32).write_unaligned(PHASE_IDENTIFIER);
        this
    }
});
