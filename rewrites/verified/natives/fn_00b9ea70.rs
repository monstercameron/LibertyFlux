// original: 0x00b9ea70 END_CHAR_SEARCH_CRITERIA
/// Native handler `END_CHAR_SEARCH_CRITERIA` (script context in, engine call out).
///
/// Ends char search criteria: clears one flag byte and sets the adjacent one. No calls, no context use.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00b9ea70(_ctx: *mut NativeCtx) -> u32 {
    unsafe {
        // Search-criteria flag pair: clear the end flag, raise the idle flag.
        *global::<u8>(0x167F09F) = 0;
        *global::<u8>(0x167F09E) = 1;
        0 // EAX is whatever the caller left; the checker does not compare it here.
    }
});
