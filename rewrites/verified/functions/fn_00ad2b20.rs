// original: 0x00ad2b20 mgr_forward_218
/// Forward one argument plus a variant word to a shared manager method.
///
/// Reads the manager object and the variant word from their globals and
/// invokes the shared callee (intercepted by the checker), returning its
/// answer.
export!(cdecl, rw_00ad2b20(arg: u32) -> u32 {
    unsafe {
        let this = *global::<u32>(0x0154E190);
        let variant = *global::<u32>(0x0154E218);
        callee_thiscall!(1, u32, this, variant, arg)
    }
});
