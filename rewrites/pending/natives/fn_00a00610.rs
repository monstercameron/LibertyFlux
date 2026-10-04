// original: 0x00a00610 ATTACH_OBJECT_TO_PED_PHYSICALLY
/// ATTACH_OBJECT_TO_PED_PHYSICALLY: forward to the shared attach unpacker.
///
/// Native handler. Pushes the call context and the attach engine address, then calls the shared unpacker that expands 15 script args into vectors.
export!(cdecl, rw_00a00610(ctx: u32) -> u32 {
    unsafe {
        // The engine address is position-dependent: derive it from the
        // relocated image base (the push site carries a HIGHLOW fixup).
        let engine = relocated(0x00A02D30);
        callee_cdecl!(1, u32, engine, ctx)
    }
});
