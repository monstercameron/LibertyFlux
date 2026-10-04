// original: 0x00b08310 reset_flag_with_lookup
/// Look up the shared object, release it, and clear the pending flag.
///
/// Asks the registry for object slot 2; when one is registered, runs its
/// single-argument release step. Always clears the module pending byte,
/// returning the release result or zero when nothing was registered.
export!(cdecl, rw_00b08310() -> u32 {
    unsafe {
        const PENDING_FLAG: u32 = 0x012B_D193;
        let obj = callee_stdcall!(1, u32, 2u32, 0u32);
        let out = if obj == 0 {
            0
        } else {
            callee_thiscall!(2, u32, obj, 0u32)
        };
        *global::<u8>(PENDING_FLAG) = 0;
        out
    }
});
