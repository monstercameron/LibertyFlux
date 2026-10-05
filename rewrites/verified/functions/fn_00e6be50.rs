// original: 0x00e6be50 veh_array_init_16x20_register
/// Call the element routine over 16 objects, then register a callback.
///
/// Passes `BASE + i * STRIDE` in ECX to the stubbed element routine
/// (thiscall/0) for `i` in `0..16` (`BASE` 0x0171B7B0, `STRIDE` 0x20),
/// then passes the code pointer 0x00E72C90 to the stubbed registrar
/// (cdecl/1, caller cleans the stack). Returns the registrar's answer.
/// Takes no arguments.
///
/// Original: 0x00E6BE50, cdecl, no arguments.
export!(cdecl, rw_00e6be50() -> u32 {
    unsafe {
        const BASE: u32 = 0x171B7B0;
        const COUNT: usize = 16;
        const STRIDE: u32 = 0x20;
        const CODEPTR: u32 = 0xE72C90;
        let mut i = 0usize;
        while i < COUNT {
            let _: u32 = callee_thiscall!(1, u32, relocated(BASE + i as u32 * STRIDE));
            i += 1;
        }
        callee_cdecl!(2, u32, relocated(CODEPTR))
    }
});
