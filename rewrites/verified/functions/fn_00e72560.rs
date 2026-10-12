// original: 0x00E72560 send_message_to_all_slots
/// Load the global slot table as the receiver and tail-call the all-slot notification routine.
lf_checker_rt::export!(cdecl, rw_fn_00E72560() -> u32 {
    const GLOBAL_TABLE: u32 = 0x01614C90;
    let table = lf_checker_rt::relocated(GLOBAL_TABLE);
    let receiver = table;
    lf_checker_rt::callee_thiscall!(1, u32, receiver)
});
