// original: 0x00e61df0 timer_block_init
// Block initialiser: publishes one record of related timer globals (two
// flag bytes cleared, the rest zeroed or set to all-ones), then hands its
// descriptor to the shared registrar and returns the registrar's answer.
lf_checker_rt::export!(cdecl, rw_00e61df0() -> u32 {
    const DESC: u32 = 0x00E707B0;
    unsafe {
        { let p = lf_checker_rt::global::<u8>(0x01B49020); p.write(p.read() & 0xFE); }
        { let p = lf_checker_rt::global::<u8>(0x01B4903C); p.write(p.read() & 0xFE); }
        lf_checker_rt::global::<u32>(0x01B49008).write(0x00000000);
        lf_checker_rt::global::<u32>(0x01B4900C).write(0xFFFFFFFF);
        lf_checker_rt::global::<u32>(0x01B49010).write(0x00000000);
        lf_checker_rt::global::<u32>(0x01B49014).write(0x00000000);
        lf_checker_rt::global::<u32>(0x01B49018).write(0x00000000);
        lf_checker_rt::global::<u32>(0x01B4901C).write(0x00000000);
        lf_checker_rt::global::<u32>(0x01B49024).write(0x00000000);
        lf_checker_rt::global::<u32>(0x01B49028).write(0xFFFFFFFF);
        lf_checker_rt::global::<u32>(0x01B4902C).write(0x00000000);
        lf_checker_rt::global::<u32>(0x01B49030).write(0x00000000);
        lf_checker_rt::global::<u32>(0x01B49034).write(0x00000000);
        lf_checker_rt::global::<u32>(0x01B49038).write(0x00000000);
    }
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(DESC))
});
