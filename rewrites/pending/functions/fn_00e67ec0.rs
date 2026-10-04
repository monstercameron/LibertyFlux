// original: 0x00e67ec0 SS_jampooltable

/// Register the model asset `SS_jampooltable` with the shared model registrar.
///
/// Calls the shared registrar (a `thiscall` taking the slot in `ecx` and the
/// name-string pointer as its one stack argument, which it pops) with this
/// member's fixed slot (`SLOT`, a word in `.data`) and fixed name
/// (`MODEL_NAME`, a NUL-terminated string in `.rdata`), and returns the
/// registrar's result unchanged.
///
/// The original takes no arguments and reads no registers; its only
/// observable behaviour is the single outgoing call and the returned `eax`.
/// Convention of this member: `cdecl` with no parameters.
lf_checker_rt::export!(cdecl, rw_00e67ec0() -> u32 {
    unsafe {
        const MODEL_NAME: u32 = 0x00E9E54C;
        const SLOT: u32 = 0x012FA61C;
        const REGISTRAR: u32 = 1;
        lf_checker_rt::callee_thiscall!(REGISTRAR, u32,
            lf_checker_rt::relocated(SLOT),
            lf_checker_rt::relocated(MODEL_NAME))
    }
});
