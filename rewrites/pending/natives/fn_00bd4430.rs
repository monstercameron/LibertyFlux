// original: 0x00bd4430 START_PTFX_ON_PED
/// Script native `START_PTFX_ON_PED` (hash 0x381C1F1C).
///
/// Passes a fixed engine routine address together with the call context itself to the engine dispatcher. No return slot is written.
export!(cdecl, rw_00bd4430(ctx: *const u8) -> u32 {
    unsafe {
        // The original pushes a routine address from its own image as an
        // ordinary argument value (a HIGHLOW-relocated push-immediate).
        // Derived via relocated(), never called or read by the rewrite.
        callee_cdecl!(1, u32, relocated(0x00bd61b0), ctx as u32)
    }
});
