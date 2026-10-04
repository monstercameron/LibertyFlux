// original: 0x00e61ef0 timer_entry_register
// Single-entry registration: passes one record pointer to the shared setter
// reached through the data-table slot, then hands its descriptor to the
// shared registrar and returns the registrar's answer.
lf_checker_rt::export!(cdecl, rw_00e61ef0() -> u32 {
    const SETTER_SLOT: u32 = 0x00E731C4;
    const RECORD: u32 = 0x01B48FE8;
    const DESC: u32 = 0x00E707F0;
    let setter: extern "stdcall" fn(u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::global::<u32>(SETTER_SLOT).read() as usize)
    };
    let _ = setter(lf_checker_rt::relocated(RECORD));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(DESC))
});
