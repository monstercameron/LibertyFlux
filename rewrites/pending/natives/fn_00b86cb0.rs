// original: 0x00b86cb0 GET_ROOT_CAM
/// Native handler `GET_ROOT_CAM`.
///
/// Forward one out-pointer to the engine.
///
/// The engine writes the camera handle through the out-pointer; not observed under interception.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_rn14_rt::export!(cdecl, rw_00b86cb0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        lf_rn14_rt::callee_cdecl!(1, u32, *args.add(0))
    }
});
