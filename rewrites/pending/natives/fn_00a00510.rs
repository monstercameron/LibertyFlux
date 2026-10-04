// original: 0x00A00510 ATTACH_OBJECT_TO_CAR_PHYSICALLY
/// Physically attaches an object to a car. Hands the call context plus the engine implementation address to the shared attach unpacker.
///
/// Native handler: the script VM passes one pointer to a call context.
/// The context holds the return-slot pointer at offset 0 and the argument
/// array pointer at offset 8. Integer arguments pass through as raw words;
/// float arguments pass through bitwise.
lf_k2_rt::export!(cdecl, rw_00a00510(ctx: *const u8) -> u32 {
    unsafe {
        // The engine implementation address is position dependent,
        // so it is derived from the relocated image base.
        let engine_fn = lf_k2_rt::relocated(0x00A02880);
        // The unpacker takes (engine_fn, ctx): the context is pushed
        // first, so the implementation address lands on top of the stack.
        lf_k2_rt::callee_cdecl!(1, u32, engine_fn, ctx as u32)
    }
});
