// original: 0x00e5cd30 init_4dc350_and_register
/// Run the init helper, then forward a fixed code pointer to the registrar.
///
/// Calls the init routine (`thiscall/0`, stubbed) forwarding entry ECX
/// untouched, then passes `0xE6E6A0` to the registrar (`cdecl/1`, stubbed).
/// Returns the registrar's answer.
export!(thiscall, rw_00e5cd30(ecx_arg: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, ecx_arg);
        callee_cdecl!(2, u32, relocated(0xE6E6A0))
    }
});
