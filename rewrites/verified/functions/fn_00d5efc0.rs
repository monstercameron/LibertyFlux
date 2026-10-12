// original: 0x00D5EFC0 material_fx_resolve_entry_references

/// Resolve a counted sequence of material entries and return its final helper result.
///
/// `this` owns a signed count at byte offset `0x52b4` and entry keys beginning
/// at `0x52b8`, with a `0x7c`-byte stride. Nonpositive counts skip the scan.
/// The scan groups adjacent entries whose key matches the previous key; each
/// key transition calls the direct pair-update helper with the prior key,
/// prior entry index, and accumulated run length. A final helper call always
/// reports the last group, including the initial sentinel group when the count
/// is nonpositive. Its scripted return value is returned in EAX. Thiscall uses
/// ECX for `this` and has no stack arguments.
lf_checker_rt::export!(thiscall, rw_material_fx_resolve_entry_references(
    this_ptr: u32
) -> u32 {
    const ENTRY_COUNT_OFFSET: u32 = 0x52B4;
    const ENTRY_LIST_OFFSET: u32 = 0x52B8;
    const ENTRY_STRIDE: u32 = 0x7C;
    const PAIR_UPDATE_HELPER: u32 = 1;
    const INITIAL_KEY: u32 = u32::MAX;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let count = unsafe { read_u32(this_ptr.wrapping_add(ENTRY_COUNT_OFFSET)) as i32 };
    let mut last_key = INITIAL_KEY;
    let mut last_index = 0u32;
    let mut run_length = 0u32;
    let mut index = 0u32;

    if count > 0 {
        while (index as i32) < count {
            let entry = this_ptr
                .wrapping_add(ENTRY_LIST_OFFSET)
                .wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let entry_key = unsafe { read_u32(entry) };
            if entry_key == last_key {
                run_length = run_length.wrapping_add(1);
            } else {
                let _ = lf_checker_rt::callee_thiscall!(
                    PAIR_UPDATE_HELPER,
                    u32,
                    this_ptr,
                    last_key,
                    last_index,
                    run_length
                );
                last_key = entry_key;
                last_index = index;
                run_length = 1;
            }
            index = index.wrapping_add(1);
        }
    }

    lf_checker_rt::callee_thiscall!(
        PAIR_UPDATE_HELPER,
        u32,
        this_ptr,
        last_key,
        last_index,
        run_length
    )
});
