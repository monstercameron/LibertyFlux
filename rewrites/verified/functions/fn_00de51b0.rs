// original: 0x00DE51B0 scrolling_row_has_item
/// Return one when the 32-bit row field at +0x1E4 is nonzero, otherwise
/// return zero. The comparison is equality against zero and the method is
/// thiscall.
lf_checker_rt::export!(thiscall, rw_de51b0(this: u32) -> u8 {
    const ITEM_STATE: u32 = 0x1E4;
    unsafe {
        u8::from(((this + ITEM_STATE) as *const u32).read_unaligned() != 0)
    }
});

