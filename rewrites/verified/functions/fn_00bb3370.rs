// original: 0x00BB3370 None
/// Calls the player creation helper with a three-word position and stores the
/// helper's EAX result through the caller-provided output pointer. The first
/// input word is unused by this wrapper. Position values are copied as raw
/// words, and the helper result is also returned in EAX.
lf_checker_rt::export!(cdecl, rw_00bb3370(
    _unused: u32, x: u32, y: u32, z: u32, result_out: u32
) -> u32 {
    const POSITION_WORDS: usize = 3;
    let position: [u32; POSITION_WORDS] = [x, y, z];
    let position_ptr = position.as_ptr() as usize as u32;
    let result = lf_checker_rt::callee_cdecl!(1, u32, position_ptr);
    unsafe { (result_out as *mut u32).write_unaligned(result); }
    result
});
