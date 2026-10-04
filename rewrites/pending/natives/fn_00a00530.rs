// original: 0x00a00530 ATTACH_OBJECT_TO_OBJECT
/// Native handler `ATTACH_OBJECT_TO_OBJECT`.
///
/// Attach one object to another with two offset vectors.
///
/// The original builds two 3-float vectors on its frame; the engine reads them as six further stack words, so the call is plain 9-word cdecl.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_rn14_rt::export!(cdecl, rw_00a00530(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        lf_rn14_rt::callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), *args.add(7), *args.add(8))
    }
});
