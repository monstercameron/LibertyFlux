// original: 0x00B8AC90 NativeImpl_GET_CUTSCENE_PED_POSITION_2

/// Copy four words from the position returned by the cutscene coordinate
/// service into the caller's output buffer.
lf_checker_rt::export!(cdecl, rw_00b8ac90(ped: u32, out_words: u32) -> () {
    let mut local_words = [0u32; 4];
    let returned_words = lf_checker_rt::callee_cdecl!(1, u32,
        local_words.as_mut_ptr() as u32, ped);
    unsafe {
        core::ptr::copy_nonoverlapping(returned_words as *const u32, out_words as *mut u32, 4);
    }
});
