// original: 0xe6b180 task_multi_init (proposed)

/// Initialise seven fixed records, then clear the ready flag.
///
/// Calls the per-record routine once for each of seven fixed
/// record addresses (object calls, no stack arguments), then
/// writes zero to the unit's flag word. No arguments, no
/// meaningful return value.
lf_checker_rt::export!(cdecl, rw_00e6b180() -> u32 {
    const THIS0: u32 = 0x016dceb8;
    const THIS1: u32 = 0x016dcfb8;
    const THIS2: u32 = 0x016dd0b8;
    const THIS3: u32 = 0x016dd1b8;
    const THIS4: u32 = 0x016dd2bc;
    const THIS5: u32 = 0x016dd3bc;
    const THIS6: u32 = 0x016dd4bc;
    const FLAG_ADDR: u32 = 0x016dd5c0;
    unsafe {
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS0));
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS1));
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS2));
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS3));
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS4));
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS5));
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS6));
        lf_checker_rt::global::<u32>(FLAG_ADDR).write_unaligned(0);
    }
    0
});
