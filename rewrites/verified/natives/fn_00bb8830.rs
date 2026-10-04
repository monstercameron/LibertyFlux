// original: 0x00bb8830 ADD_COVER_BLOCKING_AREA
/// Script native `ADD_COVER_BLOCKING_AREA` (hash 0x6E856548).
///
/// Passes the call context together with the engine routine address for this area type through the shared native unpacker. No return slot is written.
export!(cdecl, rw_00bb8830(ctx: *const u8) -> u32 {
        // Engine routine address pushed by the original. The immediate
        // carries a HIGHLOW reloc (verified in .reloc at its exact rva),
        // so the runtime value is image-base-relative.
        callee_cdecl!(1, u32, relocated(0x00BBB190), ctx as u32)
});
