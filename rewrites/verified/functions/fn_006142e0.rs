// original: 0x006142E0 streaming_sample_trace_record

/// Record a diagnostic sample when tracing is enabled and the global cursor
/// points to a destination. The three incoming scalar words come from XMM0,
/// XMM1 and XMM2. The other record words are read from global data. The record
/// is nine 32-bit words, and the cursor advances by its size only when written.
/// The global sample counter increments on every call. The token loaded from image data is returned in EAX.
lf_checker_rt::export!(cdecl, rw_006142e0() -> u32 {
    const COUNT: u32 = 0x01b4_f718;
    const ENABLED: u32 = 0x01b4_f71c;
    const CURSOR: u32 = 0x01b4_f724;
    const TOKEN: u32 = 0x0110_dfb0;
    const GLOBAL_FLOATS: [u32; 5] = [
        0x017f_59ec,
        0x017f_59f8,
        0x017f_59fc,
        0x017f_59f4,
        0x017f_59f0,
    ];
    const RECORD_BYTES: u32 = 0x24;

    let return_token = unsafe {
        let count = lf_checker_rt::global::<u32>(COUNT);
        let enabled = lf_checker_rt::global::<u32>(ENABLED);
        let cursor = lf_checker_rt::global::<u32>(CURSOR);
        let next_count = count.read_volatile().wrapping_add(1);
        let is_enabled = enabled.read_volatile() != 0;
        let record = cursor.read_volatile();
        let global_token = lf_checker_rt::global::<u32>(TOKEN).read_volatile();

        if is_enabled && record != 0 {
            let words = [
                lf_checker_rt::xmm_word(0, 0),
                lf_checker_rt::xmm_word(1, 0),
                lf_checker_rt::xmm_word(2, 0),
                lf_checker_rt::global::<u32>(GLOBAL_FLOATS[0]).read_volatile(),
                lf_checker_rt::global::<u32>(GLOBAL_FLOATS[1]).read_volatile(),
                lf_checker_rt::global::<u32>(GLOBAL_FLOATS[2]).read_volatile(),
                global_token,
                lf_checker_rt::global::<u32>(GLOBAL_FLOATS[3]).read_volatile(),
                lf_checker_rt::global::<u32>(GLOBAL_FLOATS[4]).read_volatile(),
            ];
            for (index, word) in words.into_iter().enumerate() {
                ((record + index as u32 * 4) as *mut u32).write_volatile(word);
            }
            cursor.write_volatile(record.wrapping_add(RECORD_BYTES));
        }
        count.write_volatile(next_count);
        global_token
    };
    return_token
});
