// original: 0x00DE4FE0 UIGTAScrollingRow::vf0
/// Register the callback address 0x00EFE6A8 with the cdecl registration
/// routine and return its result unchanged. The wrapper takes no arguments;
/// the callback is passed as one relocated address.
lf_checker_rt::export!(thiscall, rw_de4fe0() -> u32 {
    const CALLBACK: u32 = 0x00EFE6A8;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CALLBACK))
});

