// original: 0x00A04500 NativeImpl_GET_INTERIOR_AT_COORDS
/// Packs three raw float argument words into a stack-local coordinate vector,
/// passes its address to the object probe helper, stores the returned handle
/// through the result pointer, and returns the same handle. The pointer itself
/// differs between builds, so the contract compares a three-word call snapshot.
lf_checker_rt::export!(cdecl, rw_00A04500(x_bits: u32, y_bits: u32, z_bits: u32, result_out: u32) -> u32 {
    unsafe {
        let coordinates = [x_bits, y_bits, z_bits];
        let result = lf_checker_rt::callee_cdecl!(1, u32, coordinates.as_ptr() as u32);
        (result_out as *mut u32).write_unaligned(result);
        result
    }
});
