// original: 0x00E66810 init_array_32_strideD0_and_register (proposed)

/// Construct every entry of one global array, then register a routine.
///
/// Calls the element constructor (thiscall, element address in ecx, no
/// stack arguments) for 32 elements starting at the array base,
/// stepping 0x000000D0 bytes, then passes the handler routine's code
/// address to the registrar callee (cdecl, one argument). No arguments;
/// returns the registrar's answer in eax (cdecl).
lf_checker_rt::export!(cdecl, rw_00e66810() -> u32 {
    const BASE: u32 = 0x01292480;
    const COUNT: u32 = 32;
    const STRIDE: u32 = 0x000000D0;
    const ROUTINE: u32 = 0x00E71FA0;
    const CTOR_CALLEE: u32 = 1;
    const REGISTRAR_CALLEE: u32 = 2;
    let mut p = lf_checker_rt::relocated(BASE);
    let mut i = COUNT;
    while i != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(CTOR_CALLEE, u32, p);
        p = p.wrapping_add(STRIDE);
        i -= 1;
    }
    lf_checker_rt::callee_cdecl!(REGISTRAR_CALLEE, u32, lf_checker_rt::relocated(ROUTINE))
});
