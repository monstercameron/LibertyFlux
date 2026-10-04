// original: 0x00bd4570 TRIGGER_PTFX_ON_OBJ
/// Script native `TRIGGER_PTFX_ON_OBJ` (hash 0x50307F63).
///
/// Passes a constant engine callback address together with the script
/// call context itself to a shared effect-dispatch helper. The callback
/// address is a relocated image address (its push site carries a base
/// relocation), so it is derived from the relocated image base rather
/// than written as a literal. No return slot is written.
export!(cdecl, rw_00bd4570(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, relocated(0x00BD6510), ctx as u32)
});
