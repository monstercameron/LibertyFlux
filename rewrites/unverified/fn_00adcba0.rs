// original: 0x00ADCBA0 CRenderPhaseBlit::ctor

/// Construct a blit phase from its resource handle. Delegates base setup with
/// a zero flag, stores the handle at `this + 0x940`, installs the class table,
/// and sets the phase identifier to 5. Returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_00adcba0(this: u32, resource_handle: u32) -> u32 {
    const BASE_PHASE_CONSTRUCTOR: u32 = 1;
    const BASE_CONSTRUCTOR_FLAG: u32 = 0;
    const RESOURCE_HANDLE_OFFSET: u32 = 0x0940;
    const VTABLE_ADDRESS: u32 = 0x00EA6EB4;
    const PHASE_IDENTIFIER_OFFSET: u32 = 0x08F4;
    const PHASE_IDENTIFIER: u32 = 5;

    unsafe {
        let _base_result = lf_checker_rt::callee_thiscall!(
            BASE_PHASE_CONSTRUCTOR,
            u32,
            this,
            BASE_CONSTRUCTOR_FLAG,
        );
        (this.wrapping_add(RESOURCE_HANDLE_OFFSET) as *mut u32).write_unaligned(resource_handle);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_ADDRESS));
        (this.wrapping_add(PHASE_IDENTIFIER_OFFSET) as *mut u32).write_unaligned(PHASE_IDENTIFIER);
        this
    }
});
