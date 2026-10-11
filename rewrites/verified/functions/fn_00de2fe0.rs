// original: 0x00DE2FE0 UIScrollingMenu::vf72

/// Read the unsigned limit at `+0x1FC` and query the delegated count through virtual slot `+0x1D4`. Query the count again when the limit is above the first result or the input low byte is zero; otherwise keep the limit. Visit each index below the selected count through slot `+0x1E0`, pass the full input word to the child's slot `+0x120`, and store the input's low byte at `+0xC5`. The comparison is unsigned and the function pops one stack word.
lf_checker_rt::export!(thiscall, rw_00de2fe0(this_ptr: u32, input_word: u32) -> u32 {
    const DELEGATE_OFFSET: u32 = 0x1E0;
    const LIMIT_OFFSET: u32 = 0x1FC;
    const STORED_BYTE_OFFSET: u32 = 0xC5;
    const COUNT_QUERY: u32 = 1;
    const CHILD_AT_INDEX: u32 = 2;
    const CHILD_UPDATE: u32 = 3;
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    let delegate = unsafe { read_u32(this_ptr.wrapping_add(DELEGATE_OFFSET)) };
    let mut limit = unsafe { read_u32(this_ptr.wrapping_add(LIMIT_OFFSET)) };
    let first_count = lf_checker_rt::callee_thiscall!(COUNT_QUERY, u32, delegate);
    let mut result = first_count;
    if limit > first_count || input_word as u8 == 0 {
        limit = lf_checker_rt::callee_thiscall!(COUNT_QUERY, u32, delegate);
        result = limit;
    }
    let mut index = 0u32;
    while index < limit {
        let child = lf_checker_rt::callee_thiscall!(CHILD_AT_INDEX, u32, delegate, index);
        result = lf_checker_rt::callee_thiscall!(CHILD_UPDATE, u32, child, input_word);
        index = index.wrapping_add(1);
    }
    unsafe { ((this_ptr.wrapping_add(STORED_BYTE_OFFSET)) as *mut u8).write(input_word as u8) };
    result
});
