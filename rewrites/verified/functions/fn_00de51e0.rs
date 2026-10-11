// original: 0x00DE51E0 scrolling_row_has_previous
/// Return one exactly when the signed 32-bit row state at +0x204 is below
/// zero. Zero and positive values return zero; the method uses thiscall.
lf_checker_rt::export!(thiscall, rw_de51e0(this: u32) -> u8 {
    const NAVIGATION_STATE: u32 = 0x204;
    unsafe {
        u8::from(((this + NAVIGATION_STATE) as *const i32).read_unaligned() < 0)
    }
});

