// original: 0x00e69330 veh_pair_init_records_e69330 (proposed)
/// Initialise one vehicle entry pair, then emit one record.
///
/// Forwards two constant half-words (0x10) and a constant descriptor pointer to the
/// pair initialiser (thiscall: object in ECX, three stack words), then forwards a
/// constant record descriptor to the shared record routine (cdecl) and returns its
/// answer. Takes no arguments (cdecl, no stack words).
///
/// Original: 0x00e69330 (cdecl, no arguments; 1 thiscall + 1 cdecl call).
lf_checker_rt::export!(cdecl, rw_00e69330() -> u32 {
    unsafe {
        const OBJECT: u32 = 0x0161547C;
        const DESCRIPTOR: u32 = 0x00EAA318;
        const RECORD: u32 = 0x00E72570;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJECT), 0x10, lf_checker_rt::relocated(DESCRIPTOR), 0x10);
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(RECORD))
    }
});
