// original: 0x00e62520 net_register_type_2520
/// Register one network data type, then register this unit's handler.
///
/// Describes a type to the shared type registrar (two name pointers plus
/// the type object), then forwards this unit's handler address to the
/// common registrar and returns its result.
lf_checker_rt::export!(cdecl, rw_00e62520() -> u32 {
    let type_obj = lf_checker_rt::relocated(0x01111220);
    let name_lo = lf_checker_rt::relocated(0x00FC94B8);
    let name_hi = lf_checker_rt::relocated(0x00FC94BC);
    let _typed: u32 = lf_checker_rt::callee_thiscall!(1, u32, type_obj, name_lo, name_hi);
    let handler = lf_checker_rt::relocated(0x00E70D90);
    lf_checker_rt::callee_cdecl!(2, u32, handler)
});
