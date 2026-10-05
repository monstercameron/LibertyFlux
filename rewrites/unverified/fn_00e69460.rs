// original: 0x00e69460 veh_record_emit_e69460 (proposed)
/// Emit one vehicle initialisation record through the shared record routine.
///
/// Forwards a constant record descriptor pointer to the shared record routine (cdecl,
/// one stack word) and returns its answer. Takes no arguments (cdecl, no stack words).
///
/// Original: 0x00e69460 (cdecl, no arguments; one cdecl call of 1 stack word).
lf_checker_rt::export!(cdecl, rw_00e69460() -> u32 {
    unsafe {
        const RECORD: u32 = 0x00E72580;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(RECORD))
    }
});
