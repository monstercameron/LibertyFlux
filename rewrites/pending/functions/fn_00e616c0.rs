// original: 0x00e616c0 timing_object_reset_and_register
/// Reset the timing worker object, then register its callback.
///
/// Returns the registrar's answer.
export!(cdecl, rw_00e616c0() -> u32 {
    unsafe {
        callee_thiscall!(1, u32, relocated(0x01A01CB0));
        callee_cdecl!(2, u32, relocated(0x00E70240))
    }
});
