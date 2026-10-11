// original: 0x00DE1850 UIScrollingMenu::vf0

/// Pass the relocated static registration label to the cdecl helper and return its EAX result. The wrapper takes no arguments and performs no other memory access.
lf_checker_rt::export!(cdecl, rw_00de1850() -> u32 {
    const REGISTRATION_LABEL_VA: u32 = 0x00EFD910;
    const REGISTER_LABEL: u32 = 1;
    let label = lf_checker_rt::relocated(REGISTRATION_LABEL_VA);
    lf_checker_rt::callee_cdecl!(REGISTER_LABEL, u32, label)
});
