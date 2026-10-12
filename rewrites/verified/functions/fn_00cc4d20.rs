// original: 0x00CC4D20 check_route_identifiers
/// Call the route predicate with `(second, first)`. If that result is zero,
/// call it with `(third, first)` unless `third` is `u32::MAX`; return the last
/// predicate result. The three inputs are cdecl stack words.
lf_checker_rt::export!(cdecl, rw_00cc4d20(first: u32, second: u32, third: u32) -> u32 {
    let first_result = lf_checker_rt::callee_cdecl!(1, u32, second, first);
    if first_result != 0 || third == u32::MAX {
        first_result
    } else {
        lf_checker_rt::callee_cdecl!(1, u32, third, first)
    }
});
