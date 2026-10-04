// original: 0x00ade600 CRenderPhaseDrawScene::vf4

/// Refresh the draw-scene phase from a parameter block, then run the
/// follow-up step when the phase has an active target.
///
/// `this` is the phase object, `params` a parameter block. Forwards
/// `params` to the loader callee (id 1, thiscall with `this`), copies
/// three words from `params+0x50c/0x510/0x514` into
/// `this+0x958/0x95c/0x960`, and when the word at `this+0x954` is
/// nonzero calls the follow-up callee (id 2) with that word as its
/// object, the word at `params+0x538`, and the address `params+0x4e0`.
///
/// Edge cases: a zero word at `this+0x954` skips the second call
/// entirely; the three copied words are always written.
///
/// Original: thiscall, one stack word, returns nothing.
lf_checker_rt::export!(thiscall, rw_00ade600(this: u32, params: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 0x954;
        const DST0: u32 = 0x958;
        const DST1: u32 = 0x95c;
        const DST2: u32 = 0x960;
        const SRC0: u32 = 0x50c;
        const SRC1: u32 = 0x510;
        const SRC2: u32 = 0x514;
        const FOLLOW_ARG: u32 = 0x538;
        const FOLLOW_BUF: u32 = 0x4e0;
        lf_checker_rt::callee_thiscall!(1, u32, this, params);
        let target = ((this + TARGET) as *const u32).read_unaligned();
        let v0 = ((params + SRC0) as *const u32).read_unaligned();
        let v1 = ((params + SRC1) as *const u32).read_unaligned();
        let v2 = ((params + SRC2) as *const u32).read_unaligned();
        ((this + DST0) as *mut u32).write_unaligned(v0);
        ((this + DST1) as *mut u32).write_unaligned(v1);
        ((this + DST2) as *mut u32).write_unaligned(v2);
        if target != 0 {
            let extra = ((params + FOLLOW_ARG) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(2, u32, target, extra, params + FOLLOW_BUF);
        }
    }
    0
});
