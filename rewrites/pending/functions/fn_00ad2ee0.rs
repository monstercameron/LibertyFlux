// original: 0x00ad2ee0 mgr_forward_224
/// Forward one argument plus a variant word to a shared manager method.
///
/// Reads the manager object and the variant word from their globals and
/// invokes the shared callee (intercepted by the checker), returning its
/// answer.
export!(cdecl, rw_00ad2ee0(arg: u32) -> u32 {
    unsafe {
        let this = *global::<u32>(0x0154E190);
        let variant = *global::<u32>(0x0154E224);
        callee_thiscall!(1, u32, this, variant, arg)
    }
});
