// original: 0x00BADA90 NativeImpl_SET_PED_NON_REMOVAL_AREA_2
/// Forward six coordinate words as two three-component buffers.
///
/// Arguments 0 through 2 form the first buffer and arguments 3 through 5 form
/// the second. The buffers live only for the helper call. Values are copied as
/// raw words so signed zero and NaN payloads are preserved. The helper's EAX
/// result is returned unchanged.
lf_checker_rt::export!(cdecl, rw_00bada90(
    x0: u32, y0: u32, z0: u32, x1: u32, y1: u32, z1: u32
) -> u32 {
    const COORDINATE_WORDS: usize = 3;
    let first_corner = [x0, y0, z0];
    let second_corner = [x1, y1, z1];
    let first_ptr = first_corner.as_ptr() as usize as u32;
    let second_ptr = second_corner.as_ptr() as usize as u32;
    unsafe { lf_checker_rt::callee_cdecl!(1, u32, first_ptr, second_ptr) }
});
