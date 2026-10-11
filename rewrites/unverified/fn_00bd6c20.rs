// original: 0x00bd6c20 NativeImpl_REMOVE_PROJTEX_IN_RANGE_2

/// Package three coordinate words in a local vector and forward the vector pointer, range value, and two zero flags to the project-texture manager. The manager call is thiscall and its EAX result is returned.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd6c20(
    x_bits: u32,
    y_bits: u32,
    z_bits: u32,
    range_bits: u32,
) -> u32 {
    const REMOVE_RANGE: u32 = 1;
    const MANAGER_OBJECT: u32 = 0x013b_aba0;
    let position = [x_bits, y_bits, z_bits];
    unsafe {
        lf_checker_rt::callee_thiscall!(
            REMOVE_RANGE,
            u32,
            lf_checker_rt::relocated(MANAGER_OBJECT),
            position.as_ptr() as usize as u32,
            range_bits,
            0,
            0,
        )
    }
}
