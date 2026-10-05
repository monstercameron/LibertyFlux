// original: 0x00b9cfe0 NativeImpl_GET_SPAWN_COORDINATES_FOR_CAR_NODE_3

/// Gets spawn coordinates for a car node into a frame buffer.
///
/// Passes (`sel` - 1, wrapping) plus a frame buffer of [y, z, w, scratch]
/// (the fourth word is frame scratch, zero under the contract's
/// `stack_fill`) and two integers through the 4-argument `GET` call on
/// the global object `OBJ` as (sel-1, buf, f0, f1). Returns whatever the
/// callee returns.
///
/// The buffer pointer is a skipped call argument with snapshot-verified
/// contents (see `narrowed`).
///
/// Original: 0x00B9CFE0 (cdecl, six stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9cfe0(sel: u32, y: u32, z: u32, w: u32, f0: u32, f1: u32) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const GET: u32 = 1;
    let mut buf = [y, z, w, 0];
    lf_checker_rt::callee_thiscall!(
        GET,
        u32,
        lf_checker_rt::relocated(OBJ),
        sel.wrapping_sub(1),
        buf.as_mut_ptr() as u32,
        f0,
        f1
    )
});
