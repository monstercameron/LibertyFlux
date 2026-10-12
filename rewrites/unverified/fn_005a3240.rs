// original: 0x005A3240 Benchmark.cli

/// Select the image string for one benchmark command-line option.
///
/// This cdecl function reads one unsigned stack argument: zero selects the
/// help text, one selects the start option, two selects the report option,
/// and every other value selects the no-argument text. The returned values
/// are relocated pointers to the original image's string data.
lf_checker_rt::export!(cdecl, rw_005a3240(option: u32) -> u32 {
    const HELP_TEXT: u32 = 0x00F88A74;
    const START_TEXT: u32 = 0x00F88A60;
    const REPORT_TEXT: u32 = 0x00F88A4C;
    const DEFAULT_TEXT: u32 = 0x00F8895C;

    let text_va = match option {
        0 => HELP_TEXT,
        1 => START_TEXT,
        2 => REPORT_TEXT,
        _ => DEFAULT_TEXT,
    };
    lf_checker_rt::relocated(text_va)
});
