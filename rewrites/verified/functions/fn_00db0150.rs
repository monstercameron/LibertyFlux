// original: 0x00DB0150 network_protect_data_blob

/// Protect a caller-owned byte range with the platform data-protection API.
///
/// `input` and `input_len` form the input blob, and `output_len` points to a
/// writable word that receives the protected blob length on success. The
/// imported routine supplies a newly allocated result buffer. The wrapper
/// appends a zero byte after that result, returns its pointer, and leaves
/// `output_len` untouched when protection fails. The API call is intercepted
/// by the checker; this rewrite preserves its seven-argument stdcall shape.
lf_checker_rt::export!(cdecl, rw_00db0150(input: u32, input_len: u32, output_len: u32) -> u32 {
    const CRYPT_PROTECT_DATA: u32 = 1;

    let input_blob = [input_len, input];
    let mut output_blob = [0u32; 2];
    let succeeded = lf_checker_rt::callee_stdcall!(
        CRYPT_PROTECT_DATA,
        u32,
        input_blob.as_ptr() as u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        output_blob.as_mut_ptr() as u32,
    );
    if succeeded == 0 {
        return 0;
    }

    let protected_len = output_blob[0];
    let protected_ptr = output_blob[1];
    unsafe {
        (protected_ptr.wrapping_add(protected_len) as *mut u8).write(0);
        (output_len as *mut u32).write_unaligned(protected_len);
    }
    protected_ptr
});
