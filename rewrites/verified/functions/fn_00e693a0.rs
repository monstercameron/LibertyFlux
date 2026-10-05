// original: 0x00e693a0 PICKUP_KILLFRENZY
/// Register one named vehicle/pickup entry with the shared registrar.
///
/// Forwards a constant entry descriptor (string) pointer and a constant object pointer
/// to the shared registrar routine (thiscall: object in ECX, descriptor on the stack)
/// and returns its answer. Takes no arguments and reads no caller state (cdecl, no
/// stack words).
///
/// Original: 0x00e693a0 (cdecl, no arguments; one thiscall of 1 stack word).
lf_checker_rt::export!(cdecl, rw_00e693a0() -> u32 {
    unsafe {
        const DESCRIPTOR: u32 = 0x00EAA78C;
        const OBJECT: u32 = 0x01632BE0;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJECT), lf_checker_rt::relocated(DESCRIPTOR))
    }
});
