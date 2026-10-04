// original: 0x00b9f710 IS_ANY_CHAR_SHOOTING_IN_AREA
/// Script native `IS_ANY_CHAR_SHOOTING_IN_AREA` (hash 0x19D16ACE).
///
/// Passes two words to the engine: a code address baked into the instruction
/// stream as an immediate (a predicate callback; the immediate carries a
/// base relocation, so it is derived with `relocated()` exactly as the
/// loader-mapped original sees it) and the call context pointer itself,
/// which carries the area bounds. No return slot is written; the engine
/// answer is the exit value.
export!(cdecl, rw_00b9f710(ctx: *const u8) -> u32 {
    const AREA_PREDICATE_FILE_VA: u32 = 0x00BA81A0;
    callee_cdecl!(1, u32, relocated(AREA_PREDICATE_FILE_VA), ctx as u32)
});
