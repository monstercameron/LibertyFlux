// original: 0x00B97360 NativeImpl_SET_UP_TRIP_SKIP_FOR_SPECIFIC_VEHICLE

/// Resolve the handle through the shared pool context, then submit the
/// three position words, final float, zero flag, resolved value and trailing
/// zero to the script helper. Return the helper's EAX value.
lf_checker_rt::export!(cdecl, rw_00b97360(x: u32, y: u32, z: u32, tail: u32, handle: u32) -> u32 {
    const POOL_CONTEXT_VA: u32 = 0x012E_22A4;
    const LOOKUP: u32 = 1;
    const SUBMIT: u32 = 2;

    let pool_context = unsafe {
        (lf_checker_rt::global::<u32>(POOL_CONTEXT_VA) as *const u32).read_unaligned()
    };
    let resolved = lf_checker_rt::callee_thiscall!(LOOKUP, u32, pool_context, handle);
    let frame = [x, y, z];
    lf_checker_rt::callee_cdecl!(
        SUBMIT,
        u32,
        frame.as_ptr() as u32,
        tail,
        0,
        resolved,
        0,
    )
});
