// original: 0x006A58E0 clear_two_qword_slots

#[allow(unused_doc_comments)]
/// Clears the two eight-byte slots at offsets `0x80` and `0x88` from the
/// object pointer and returns that pointer unchanged. The function takes its
/// object pointer in ECX using the 32-bit thiscall convention; it does not
/// inspect the prior slot contents or call other functions.
lf_checker_rt::export!(thiscall, rw_006a58e0(this: u32) -> u32 {
    const FIRST_ZEROED_QWORD: u32 = 0x80;
    const SECOND_ZEROED_QWORD: u32 = 0x88;

    unsafe {
        (this.wrapping_add(FIRST_ZEROED_QWORD) as *mut u64).write_unaligned(0);
        (this.wrapping_add(SECOND_ZEROED_QWORD) as *mut u64).write_unaligned(0);
    }
    this
});
