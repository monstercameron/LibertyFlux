// original: 0x00e69430 veh_entry_register_e69430 (proposed)
/// Register one named vehicle/pickup entry with the shared registrar.
///
/// Forwards a constant entry descriptor (string) pointer and a constant object pointer
/// to the shared registrar routine (thiscall: object in ECX, descriptor on the stack)
/// and returns its answer. Takes no arguments and reads no caller state (cdecl, no
/// stack words).
///
/// Original: 0x00e69430 (cdecl, no arguments; one thiscall of 1 stack word).
lf_checker_rt::export!(cdecl, rw_00e69430() -> u32 {
    unsafe {
        const DESCRIPTOR: u32 = 0x00EAAF84;
        const OBJECT: u32 = 0x016334A0;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJECT), lf_checker_rt::relocated(DESCRIPTOR))
    }
});
