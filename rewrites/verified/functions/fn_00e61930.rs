// original: 0x00e61930 timing_group_init_and_register
/// Initialise the timer group header, then register the handler.
///
/// Runs the group initialiser (stubbed; it pops 12 bytes of caller scratch, stdcall/3 with uninitialised arguments whose values are skipped in the contract), stores the relocated image pointer `0x00FE581C` at `0x01A057D4`, then passes the code pointer `0x00E703F0` to the registrar helper (stubbed, cdecl/1) and returns its answer.
export!(cdecl, rw_00e61930() -> u32 {
    unsafe {
        let _: u32 = callee_stdcall!(0, u32, 0, 0, 0);
        *global::<u32>(0x01a057d4) = relocated(0x00fe581c);
        callee_cdecl!(1, u32, relocated(0x00e703f0))
    }
});
