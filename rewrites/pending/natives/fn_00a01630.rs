// original: 0x00a01630 IS_OBJECT_UPRIGHT
/// Reports whether an object is upright within a float tolerance. Stores the engine's boolean answer in the return slot.
///
/// Native handler: the script VM passes one pointer to a call context.
/// The context holds the return-slot pointer at offset 0 and the argument
/// array pointer at offset 8. Integer arguments pass through as raw words;
/// float arguments pass through bitwise.
checker_rt::export!(cdecl, rw_00a01630(ctx: *const u8) -> u32 {
    unsafe {
        let argv = *((ctx.add(8)) as *const *const u32);
        let a0 = *argv.add(0);
        let f1 = *((argv.add(1)) as *const f32);
        let answer: u32 = checker_rt::callee_cdecl!(1, u32, a0, f32::to_bits(f1));
        // Only the low byte of the engine answer is defined; the rest is discarded.
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
