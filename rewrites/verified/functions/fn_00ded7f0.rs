// original: 0x00DED7F0 UIButton::vf0

/// Return the scripted result of hashing the button's static type-name string.
/// The string address is relocated from its image-relative value before it is
/// passed to the intercepted cdecl helper.
lf_checker_rt::export!(thiscall, rw_00ded7f0(_this: u32) -> u32 {
    unsafe {
        const TYPE_NAME: u32 = 0x00f005b8;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(TYPE_NAME))
    }
});
