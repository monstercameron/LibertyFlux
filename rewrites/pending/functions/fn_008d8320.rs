// original: 0x008d8320 slot_query
/// Zero two out-words, then query a slot through the engine helper.
///
/// Combines the table entry for index `a2` with the base `a1`, zeroes both
/// caller words, and forwards `(combined, a3, a4, a5, a6)` to a cdecl/5
/// callee; returns its answer.
export!(cdecl, rw_008d8320(a1: u32, a2: u32, a3: *mut u32, a4: *mut u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        let combined = (*global::<u32>(0x013053A8)
            .byte_add(a2.wrapping_mul(100) as usize)).wrapping_add(a1);
        *a3 = 0;
        *a4 = 0;
        callee_cdecl!(1, u32, combined, a3 as u32, a4 as u32, a5, a6)
    }
});
