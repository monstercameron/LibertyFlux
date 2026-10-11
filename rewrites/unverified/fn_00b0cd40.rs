// original: 0x00B0CD40 Pickup_RegisterGlobalHandler

/// Register the pickup subsystem's three callback functions with its global
/// handler registry. The registry object and callback pointers are relocated
/// from their preferred image addresses. The callback order is preserved.
///
/// Calling convention: cdecl with no arguments and no return value. The
/// registry helper is invoked once as a thiscall with three stack arguments.
lf_checker_rt::export!(cdecl, rw_00B0CD40() -> u32 {
    const REGISTRY_OBJECT_VA: u32 = 0x012b_cd18;
    const PICKUP_EVENT_CALLBACK_VA: u32 = 0x00b0_e5e0;
    const PICKUP_REMOVE_CALLBACK_VA: u32 = 0x00b0_d030;
    const PICKUP_EVENT_TABLE_VA: u32 = 0x00ea_a7bc;

    let registry = lf_checker_rt::relocated(REGISTRY_OBJECT_VA);
    let _ = lf_checker_rt::callee_thiscall!(
        1,
        u32,
        registry,
        lf_checker_rt::relocated(PICKUP_EVENT_TABLE_VA),
        lf_checker_rt::relocated(PICKUP_REMOVE_CALLBACK_VA),
        lf_checker_rt::relocated(PICKUP_EVENT_CALLBACK_VA)
    );
    0
});
