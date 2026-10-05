// original: 0x008949c0 audio_index_from_ptr_thunk
/// Pass-through thunk to a shared implementation.
///
/// Takes one word on the stack and hands it to the shared implementation
/// unchanged, returning whatever that call answers. Control never comes
/// back here: the transfer at the entry leaves this address for good. The
/// bytes after the entry hold padding, not code; where the inventory size
/// runs past the padding, only this thunk is rewritten, not the neighbours.
export!(cdecl, rw_008949c0(ctx: u32) -> u32 {
    callee_cdecl!(1, u32, ctx)
});
