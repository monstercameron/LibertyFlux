// original: 0x00E69210 veh_bank_array_init (proposed)
/// Initialise a bank of vehicle slots, then register a callback.
///
/// Calls the bank callee (`BANK_CALLEE`, thiscall, object pointer in `ecx`,
/// no stack arguments) `COUNT` times with the addresses `BASE + i *
/// STRIDE`, then passes the static code block `ARG` to the registrar callee
/// (`REG_CALLEE`, cdecl, one argument).
///
/// Original: 0x00E69210 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e69210() -> u32 {
    unsafe {
        const BASE: u32 = 0x015F8BC0;
        const COUNT: u32 = 60;
        const STRIDE: u32 = 0x110;
        const ARG: u32 = 0x00E72550;
        const BANK_CALLEE: u32 = 1;
        const REG_CALLEE: u32 = 2;
        let mut p = lf_checker_rt::relocated(BASE);
        let mut i = 0u32;
        while i < COUNT {
            lf_checker_rt::callee_thiscall!(BANK_CALLEE, u32, p);
            p = p.wrapping_add(STRIDE);
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(REG_CALLEE, u32,
            lf_checker_rt::relocated(ARG));
        0
    }
});
