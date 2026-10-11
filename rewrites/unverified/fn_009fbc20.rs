// original: 0x009fbc20 CPlayStatInt::CPlayStatInt_4

/// Builds a temporary integer play-stat object, applies the base layout, stores its relocated descriptor pointer, resolves the selected triplet, packs the auxiliary pair, emits the 0x38-byte object, destroys the base, and performs the stack-cookie check. The temporary frame is zero-filled to match the contract's defined stack fill.
lf_checker_rt::export!(cdecl, rw_009fbc20() -> u32 {
    const DESCRIPTOR_VA: u32 = 0x00e99878;
    const TEMP_WORD_OFFSET: usize = 5;
    const AUX_WORD_OFFSET: usize = 18;
    const EMIT_BYTES: u32 = 0x38;
    unsafe {
        let mut frame = [0u32; 24];
        let frame_base = frame.as_mut_ptr();
        let temporary = frame_base.add(TEMP_WORD_OFFSET);
        let auxiliary = frame_base.add(AUX_WORD_OFFSET);

        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, temporary as u32, 1, 1);
        temporary.write(lf_checker_rt::relocated(DESCRIPTOR_VA));
        let selected_triplet = lf_checker_rt::callee_cdecl!(2, u32, frame_base as u32);
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, selected_triplet, auxiliary as u32);
        let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, temporary as u32, EMIT_BYTES);
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, temporary as u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, 0);
        0
    }
});
