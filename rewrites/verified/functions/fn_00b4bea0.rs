// original: 0x00b4bea0 peds_in_same_group

/// Compare the group returned by the first ped lookup with the other ped's
/// group. A null first lookup returns false; otherwise the lookup runs again
/// for the first ped and once for the second. The helper result is compared
/// for exact 32-bit equality. The function is cdecl with two stack pointers;
/// its AL result preserves the upper 24 bits from the final helper result.
lf_checker_rt::export!(cdecl, rw_00B4BEA0(first_ped: u32, second_ped: u32) -> u32 {
    const NULL_GROUP: u32 = 0;
    let first_group = lf_checker_rt::callee_thiscall!(1, u32, first_ped);
    if first_group == NULL_GROUP { return first_group & !0xFF; }

    let first_group_again = lf_checker_rt::callee_thiscall!(1, u32, first_ped);
    let second_group = lf_checker_rt::callee_thiscall!(1, u32, second_ped);
    (second_group & !0xFF) | u32::from(first_group_again == second_group)
});
