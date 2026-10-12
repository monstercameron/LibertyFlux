// original: 0x00BAEA50 NativeImpl_GET_CHAR_LAST_DAMAGE_BONE_2
/// Resolve a character, test its signed damage-bone count, and write a query result.
///
/// The second cdecl argument points to a 32-bit output word, which is cleared
/// before lookup. A null character or a signed count less than or equal to
/// zero returns false with that word left at zero. A positive count is passed
/// with the character pointer to the query helper; its EAX result is stored
/// through the output pointer and the function returns true in AL.
lf_checker_rt::export!(cdecl, rw_baea50(
    handle: u32, result_out: u32
) -> u32 {
    const DAMAGE_COUNT_OFFSET: usize = 2680;
    let output = result_out as usize as *mut u32;
    unsafe { output.write_unaligned(0) };

    let character = lf_checker_rt::callee_thiscall!(1, u32, 0u32, handle);
    if character == 0 { return 0; }
    let count = unsafe { ((character as usize + DAMAGE_COUNT_OFFSET) as *const u32).read_unaligned() };
    if (count as i32) <= 0 { return 0; }

    let query_result = lf_checker_rt::callee_cdecl!(2, u32, character, count);
    unsafe { output.write_unaligned(query_result) };
    1
});
