// original: 0x00922B70 prepare_render_parameters_and_submit

/// Build a 16-byte parameter record and submit it with two input floats.
///
/// The five cdecl arguments are two float bit patterns, an optional target
/// pointer, and two words copied into the parameter record. A local record
/// starts with four zero words. The first thiscall helper writes a relocated
/// data pointer, the two trailing arguments, and a zero into that record. The
/// second helper receives the first two float words, the target pointer, and
/// a pointer to the completed record; its EAX result is returned unchanged.
///
/// Calling convention: cdecl with five stack words. The record is local to
/// this call; the contract compares its four words at the second helper call.
lf_checker_rt::export!(cdecl, rw_00922B70(
    first_float_bits: u32,
    second_float_bits: u32,
    target: u32,
    record_word_one: u32,
    record_word_two: u32,
) -> u32 {
    const INITIAL_RECORD_WORDS: [u32; 4] = [0; 4];
    const SET_RECORD_CALLEE: u32 = 1;
    const SUBMIT_RECORD_CALLEE: u32 = 2;

    let mut record = INITIAL_RECORD_WORDS;
    let record_ptr = record.as_mut_ptr() as usize as u32;
    let _record_pointer = lf_checker_rt::callee_thiscall!(
        SET_RECORD_CALLEE,
        u32,
        record_ptr,
        record_word_one,
        record_word_two,
    );

    lf_checker_rt::callee_cdecl!(
        SUBMIT_RECORD_CALLEE,
        u32,
        first_float_bits,
        second_float_bits,
        target,
        record_ptr,
    )
});
