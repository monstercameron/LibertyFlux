// original: 0x00953750 lookup_field_30
/// Resolve the shared handle through the registry and read its field at +0x30.
export!(cdecl, rw_00953750() -> u32 {
    unsafe {
        let arg = *global::<u32>(0x11F6F34);
        let this = *global::<u32>(0x11F6954);
        let resolved: u32 = callee_thiscall!(0, u32, this, arg);
        *((resolved.wrapping_add(0x30)) as *const u32)
    }
});
