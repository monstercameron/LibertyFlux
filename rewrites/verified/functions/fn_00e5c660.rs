// original: 0x00e5c660 zero_counters_then_register
/// Clear this unit's counter block, then register its handler.
/// Returns the registrar's answer.
export!(cdecl, rw_00e5c660() -> u32 {
    unsafe {
        global::<u32>(0x018DDE20).write(0);
        global::<u32>(0x018DDE24).write(0);
        global::<u32>(0x018DDE28).write(0);
        global::<u32>(0x018DDE3C).write(0);
        global::<u32>(0x018DDE40).write(0);
        global::<u32>(0x018DDE44).write(0);
        global::<u32>(0x018DDE58).write(0);
        global::<u32>(0x018DDE5C).write(0);
        global::<u32>(0x018DDE60).write(0);
        global::<u32>(0x018DDE74).write(0);
        global::<u32>(0x018DDE78).write(0);
        global::<u32>(0x018DDE7C).write(0);
        global::<u32>(0x018DDE90).write(0);
        global::<u32>(0x018DDE94).write(0);
        global::<u32>(0x018DDE98).write(0);
    }
    lf_checker_rt::callee_cdecl!(1, u32, relocated(0x00E6E390))
});
