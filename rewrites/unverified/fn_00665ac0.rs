// original: 0x00665AC0 network_record_text_pointer

/// Return the pointer to a record's text area, located 0x24 bytes from the
/// record start. A null record remains null; this cdecl helper does not access
/// the record or mutate memory.
lf_checker_rt::export!(cdecl, rw_00665AC0(record: u32) -> u32 {
    const TEXT_OFFSET: u32 = 0x24;
    if record == 0 { 0 } else { record.wrapping_add(TEXT_OFFSET) }
});
