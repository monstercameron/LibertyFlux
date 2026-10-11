// original: 0x00DE51F0 scrolling_row_has_next
/// Return one exactly when the signed 32-bit row state at +0x204 is above
/// zero. Zero and negative values return zero; the method uses thiscall.
lf_checker_rt::export!(thiscall, rw_de51f0(this: u32) -> u8 {
    const NAVIGATION_STATE: u32 = 0x204;
    unsafe {
        u8::from(((this + NAVIGATION_STATE) as *const i32).read_unaligned() > 0)
    }
});

