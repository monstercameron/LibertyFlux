// original: 0x0086f170 PRINTNL
/// Script-native handler thunk through a dispatch slot.
///
/// Takes one word, the script context pointer. Hands that pointer to the
/// implementation whose address the dispatch slot currently holds, and
/// returns whatever that call answers. Control never comes back here:
/// the indirect transfer at the entry leaves this address for good. The
/// bytes after the entry hold padding, not code. The checker cannot patch
/// an indirect transfer site, so the contract plants its stand-in address
/// in the slot and this rewrite calls that stand-in.
export!(cdecl, rw_0086f170(ctx: u32) -> u32 {{
    callee_cdecl!(1, u32, ctx)
}});
