// original: 0x00e612f0 submit_with_frame_args
/// submit_with_frame_args: run the frame-args initializer, stamp the record, submit.
///
/// The original reserves 12 bytes of scratch that the callee pops
/// (stdcall/3) without reading; the rewrite passes plain zeros for
/// those unread words. Then it stamps the record id (a relocated
/// pointer constant) and hands this pair's stub to the shared
/// submit routine, returning its answer.
lf_checker_rt::export!(cdecl, rw_00e612f0() -> u32 {
    unsafe {
        lf_checker_rt::callee_stdcall!(1, u32, 0, 0, 0);
        lf_checker_rt::global::<u32>(0x01A00C20).write(lf_checker_rt::relocated(0x00FE4EB4));
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E70010))
    }
});
