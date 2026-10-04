// original: 0x00add1a0 CRenderPhaseDrawScene::vf0

/// Scalar deleting destructor of the draw-scene render phase.
///
/// `this` is the object; `flags` is the standard destructor flag word.
/// Runs the (intercepted) destructor, then frees the object with the
/// scalar `operator delete` callee when bit 0 of `flags` is set.
/// Returns `this` unchanged in all cases.
///
/// Edge cases: any flag value with bit 0 clear skips the delete call;
/// only bit 0 is tested, all other bits are ignored.
///
/// Original: thiscall, one stack word, callee id 1 is the destructor
/// (thiscall, no stack arguments), callee id 2 is `operator delete`
/// (cdecl, one argument).
lf_checker_rt::export!(thiscall, rw_00add1a0(this: u32, flags: u32) -> u32 {
    const DELETE_FLAG: u32 = 1;
    lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & DELETE_FLAG != 0 {
        lf_checker_rt::callee_cdecl!(2, u32, this);
    }
    this
});
