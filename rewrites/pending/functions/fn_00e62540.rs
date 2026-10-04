// original: 0x00e62540 net_register_type_2540
/// Register one network data type, then register this unit's handler.
///
/// Describes a type to the shared type registrar (two name pointers plus
/// the type object), then forwards this unit's handler address to the
/// common registrar and returns its result.
lf_checker_rt::export!(cdecl, rw_00e62540() -> u32 {
    let type_obj = lf_checker_rt::relocated(0x01111208);
    let name_lo = lf_checker_rt::relocated(0x00FC94C0);
    let name_hi = lf_checker_rt::relocated(0x00FC94C5);
    let _typed: u32 = lf_checker_rt::callee_thiscall!(1, u32, type_obj, name_lo, name_hi);
    let handler = lf_checker_rt::relocated(0x00E70DD0);
    lf_checker_rt::callee_cdecl!(2, u32, handler)
});
