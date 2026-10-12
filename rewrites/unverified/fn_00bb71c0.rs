// original: 0x00BB71C0 None
/// Forward three coordinate words through a temporary vector buffer.
///
/// The helper receives a pointer to the three words in argument order. The
/// local buffer is valid only for the duration of that call, and the helper's
/// EAX result is returned unchanged. Copying raw words preserves float bits.
lf_checker_rt::export!(cdecl, rw_00bb71c0(x: u32, y: u32, z: u32) -> u32 {
    const VECTOR_WORDS: usize = 3;
    let vector: [u32; VECTOR_WORDS] = [x, y, z];
    let vector_ptr = vector.as_ptr() as usize as u32;
    lf_checker_rt::callee_cdecl!(1, u32, vector_ptr)
});
