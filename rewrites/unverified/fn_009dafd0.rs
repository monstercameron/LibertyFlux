// original: 0x009DAFD0 streaming_check_index_with_fallback_b

/// Read the second category word from a fixed global, then ask a helper whether
/// the index belongs to that category. A nonzero low byte selects the category
/// handler and returns its EAX; otherwise, query the index state and invoke the
/// fallback only when that full EAX value is nonzero. Calls are cdecl.
lf_checker_rt::export!(cdecl, rw_009dafd0(index: u32) -> u32 {
    const CATEGORY_GLOBAL: u32 = 0x0104_b888;
    let category = unsafe { lf_checker_rt::global::<u32>(CATEGORY_GLOBAL).read_unaligned() };

    let classified = lf_checker_rt::callee_cdecl!(1, u32, index, category);
    if classified as u8 != 0 {
        return lf_checker_rt::callee_cdecl!(2, u32, index, category);
    }

    let active = lf_checker_rt::callee_cdecl!(3, u32, index);
    if active != 0 {
        lf_checker_rt::callee_cdecl!(4, u32, index)
    } else {
        active
    }
});
