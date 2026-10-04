// original: 0x00DDE110 select_inner_object
/// Select one of the two inner objects: the secondary one at +0x1EC when
/// `which` is 1, otherwise the primary one at +0x1E8.
lf_checker_rt::export!(thiscall, rw_dde110(this: u32, which: u32) -> u32 {
    unsafe {
        if which == 1 {
            *((this + 0x1EC) as *const u32)
        } else {
            *((this + 0x1E8) as *const u32)
        }
    }
});
