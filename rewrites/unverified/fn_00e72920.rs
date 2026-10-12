// original: 0x00E72920 clear_navigation_work_tables
/// Load the global navigation work table as the receiver and tail-call its cleanup routine.
lf_checker_rt::export!(cdecl, rw_fn_00E72920() -> u32 {
    const GLOBAL_TABLE: u32 = 0x016C6830;
    let table = lf_checker_rt::relocated(GLOBAL_TABLE);
    let receiver = table;
    lf_checker_rt::callee_thiscall!(1, u32, receiver)
});
