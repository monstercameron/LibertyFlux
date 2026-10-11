// original: 0x006F7F10 mainloop_timer_mode_guard

/// Return false immediately for modes other than zero and one. For the two
/// active modes, read the indexed timer's unsigned count, return false when it
/// is zero, then query the timer and conditionally call the text-buffer helper.
/// Return true only after that helper call. The test contract exercises the
/// early unsupported-mode guard; the indexed table and helper path are outside
/// that proof.
lf_checker_rt::export!(fastcall, rw_006f7f10(category: u32, mode: u32, buffer: u32) -> u8 {
    const TABLE_FILE_VA: u32 = 0x019fbc60;
    const TABLE_STRIDE: u32 = 0x0e70;
    const COUNT_OFFSET: u32 = 0x280;
    const TIMER_QUERY_CALLEE: u32 = 1;
    const TEXT_BUFFER_CALLEE: u32 = 2;

    if mode != 0 && mode != 1 {
        return 0;
    }

    let table_entry = TABLE_FILE_VA.wrapping_add(category.wrapping_mul(TABLE_STRIDE));
    let count = unsafe { lf_checker_rt::global::<u32>(table_entry + COUNT_OFFSET).read_unaligned() };
    if count == 0 {
        return 0;
    }

    let table_pointer = lf_checker_rt::relocated(table_entry);
    let timer_value: u32 = lf_checker_rt::callee_thiscall!(TIMER_QUERY_CALLEE, u32, table_pointer);
    if timer_value | mode == 0 {
        return 0;
    }

    let _text_result: u32 = lf_checker_rt::callee_thiscall!(TEXT_BUFFER_CALLEE, u32, buffer, timer_value, mode);
    1
});
