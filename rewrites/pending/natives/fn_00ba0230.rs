// original: 0x00ba0230 IS_PED_RAGDOLL
/// Script native handler `IS_PED_RAGDOLL`.
///
/// Returns true if the character is in ragdoll mode.
///
/// Forwards 1 script argument to the engine function and stores the
/// low byte of its answer (zero-extended) into the return slot.
/// handler function: `0x00ba0230`, engine call site: `0x00ba023a`.
export!(cdecl, rw_ba0230(ctx: *const NativeCtx03) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        let answer: u32 = callee_cdecl!(1, u32, a0);
        *(*ctx).ret_ptr = answer & 0xFF;
        (*ctx).ret_ptr as u32
    }
});
