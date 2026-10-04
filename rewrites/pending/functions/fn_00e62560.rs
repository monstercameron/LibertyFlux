// original: 0x00e62560 net_register_type_2560
/// Register one network data type, then register this unit's handler.
///
/// Describes a type to the shared type registrar (two name pointers plus
/// the type object), then forwards this unit's handler address to the
/// common registrar and returns its result.
lf_checker_rt::export!(cdecl, rw_00e62560() -> u32 {
    let type_obj = lf_checker_rt::relocated(0x011111F0);
    let name_lo = lf_checker_rt::relocated(0x00F1C68F);
    let name_hi = lf_checker_rt::relocated(0x00F1C68F);
    let _typed: u32 = lf_checker_rt::callee_thiscall!(1, u32, type_obj, name_lo, name_hi);
    let handler = lf_checker_rt::relocated(0x00E70E10);
    lf_checker_rt::callee_cdecl!(2, u32, handler)
});
