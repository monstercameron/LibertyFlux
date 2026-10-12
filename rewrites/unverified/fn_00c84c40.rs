// original: 0x00C84C40 scrollbars_dat_find_entry

/// Search the runtime scrollbar configuration for an entry whose type word
/// is `0x2C`. The entry count is the unsigned 16-bit word at root offset `+4`;
/// entries begin at table offset `+0x80` with stride `0x88`. The selected
/// entry, or the embedded default entry at root offset `+0x690`, is passed to
/// the parser unless its sentinel at `+0x84` is `u32::MAX`, in which case the
/// fallback data pointer is used. The parser result is returned unchanged.
lf_checker_rt::export!(cdecl, rw_00c84c40() -> u32 {
    const CONFIG_ROOT_GLOBAL: u32 = 0x01BB_5628;
    const TABLE_POINTER_OFFSET: u32 = 0;
    const ENTRY_COUNT_OFFSET: u32 = 4;
    const ENTRY_START_OFFSET: u32 = 0x80;
    const ENTRY_STRIDE: u32 = 0x88;
    const ENTRY_TYPE_OFFSET: u32 = 0;
    const ENTRY_SENTINEL_OFFSET: u32 = 0x84;
    const DEFAULT_ENTRY_OFFSET: u32 = 0x690;
    const FALLBACK_DATA_VA: u32 = 0x00ED_6174;
    const MATCH_TYPE: u32 = 0x2C;
    const PARSE_SCROLLBAR_DATA: u32 = 1;
    const SENTINEL: u32 = u32::MAX;

    let config = unsafe { lf_checker_rt::global::<u32>(CONFIG_ROOT_GLOBAL).read_unaligned() };
    let table = unsafe {
        (config.wrapping_add(TABLE_POINTER_OFFSET) as *const u32).read_unaligned()
    };
    let entry_count = unsafe {
        (config.wrapping_add(ENTRY_COUNT_OFFSET) as *const u16).read_unaligned() as u32
    };

    let mut selected_entry = 0;
    for index in 0..entry_count {
        let candidate = table.wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
        let entry_type = unsafe {
            (candidate.wrapping_add(ENTRY_START_OFFSET).wrapping_add(ENTRY_TYPE_OFFSET)
                as *const u32)
                .read_unaligned()
        };
        if entry_type == MATCH_TYPE {
            selected_entry = candidate;
            break;
        }
    }

    let data_entry = if selected_entry == 0 {
        config.wrapping_add(DEFAULT_ENTRY_OFFSET)
    } else {
        selected_entry
    };
    let sentinel = unsafe {
        (data_entry.wrapping_add(ENTRY_SENTINEL_OFFSET) as *const u32).read_unaligned()
    };
    let parser_argument = if sentinel == SENTINEL {
        lf_checker_rt::relocated(FALLBACK_DATA_VA)
    } else {
        data_entry
    };
    lf_checker_rt::callee_cdecl!(PARSE_SCROLLBAR_DATA, u32, parser_argument)
});
