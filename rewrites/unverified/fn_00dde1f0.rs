// original: 0x00DDE1F0 get_inner_limit
/// Return the limit value held by the inner object: the word at +0x1F4 of
/// the object at +0x1E8.
lf_checker_rt::export!(thiscall, rw_dde1f0(this: u32) -> u32 {
    unsafe {
        let inner = *((this + 0x1E8) as *const u32);
        *((inner + 0x1F4) as *const u32)
    }
});
