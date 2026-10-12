// original: 0x00E713E0 release_audio_object_array_resources
/// Release two global audio resources in order; the second release is the terminal tail call.
lf_checker_rt::export!(cdecl, rw_fn_00E713E0() -> () {
    unsafe {
        const FIRST_RESOURCE: u32 = 0x010344D4;
        const SECOND_RESOURCE: u32 = 0x010344D0;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(FIRST_RESOURCE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(SECOND_RESOURCE));
    }
});
