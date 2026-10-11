// original: 0x009DAEE0 streaming_forward_two_parameter_pairs

/// Forward the first two cdecl arguments to a preparatory helper, then forward
/// the first and third arguments to a second helper. The second helper's EAX
/// result is the function result; the first helper's result is ignored.
lf_checker_rt::export!(cdecl, rw_009daee0(first: u32, second: u32, third: u32) -> u32 {
    const FIRST_HELPER: u32 = 1;
    const SECOND_HELPER: u32 = 2;

    let _forwarded = lf_checker_rt::callee_cdecl!(FIRST_HELPER, u32, first, second);
    lf_checker_rt::callee_cdecl!(SECOND_HELPER, u32, first, third)
});
