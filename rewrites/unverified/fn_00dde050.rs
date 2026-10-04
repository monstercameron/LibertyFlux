// original: 0x00DDE050 MO_TEXT_FIELD
/// Build a text field's five Metrics/scales via the style lookup, then
/// create the backing field object from the collected values.
///
/// Each lookup resolves one style id (62, 60, 1, 2, 62) into a caller-side
/// slot; the five answers plus two stock string pointers, the default
/// 15.0/30.0 extents and four zero words form the thirteen-word argument
/// frame of the final creation call, whose answer is returned.
/// The slot buffers are pure call scratch: the callee's stores into them
/// are never read back, so only the ids and answers are compared.
lf_checker_rt::export!(thiscall, rw_dde050(this: u32) -> u32 {
    let mut slot0 = 0u32;
    let mut slot1 = 0u32;
    let mut slot2 = 0u32;
    let mut slot3 = 0u32;
    let mut slot4 = 0u32;
    let r1 = lf_checker_rt::callee_cdecl!(1, u32, core::ptr::addr_of_mut!(slot0) as u32, 0x3E);
    let r2 = lf_checker_rt::callee_cdecl!(1, u32, core::ptr::addr_of_mut!(slot1) as u32, 0x3C);
    let r3 = lf_checker_rt::callee_cdecl!(1, u32, core::ptr::addr_of_mut!(slot2) as u32, 1);
    let r4 = lf_checker_rt::callee_cdecl!(1, u32, core::ptr::addr_of_mut!(slot3) as u32, 2);
    let r5 = lf_checker_rt::callee_cdecl!(1, u32, core::ptr::addr_of_mut!(slot4) as u32, 0x3E);
    lf_checker_rt::callee_thiscall!(
        2, u32, this, r5, r4, r3, r2, r1,
        lf_checker_rt::relocated(0x00EFCDF1),
        lf_checker_rt::relocated(0x00EFCF64),
        0x41700000, 0x41F00000, 0, 0, 0, 0
    )
});
