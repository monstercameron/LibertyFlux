// original: 0x00e5d1c0 subsys_init_and_register
/// Initialise the subsystem object, then forward a code pointer to the registrar.
///
/// Calls the init routine (`thiscall/3`, stubbed) with ECX pointing at the
/// object at `0x0198B758` and arguments `(0x1B4F7B0, 0x64000, 1)`, then passes
/// `0x00E6E7C0` to the registrar (`cdecl/1`, stubbed). Returns the
/// registrar's answer.
export!(cdecl, rw_00e5d1c0() -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, relocated(0x198B758),
            relocated(0x1B4F7B0), 0x64000, 1);
        callee_cdecl!(2, u32, relocated(0xE6E7C0))
    }
});
