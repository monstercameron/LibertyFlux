// original: 0x009e8530 ped_link_float_field
/// Float at `+8` of the object linked at `+0xA80`, returned on the
/// x87 stack. (thiscall; the sibling word at `+4` is staged but never read.)
lf_checker_rt::export!(thiscall, rw_009e8530(this_ptr: u32) -> f32 {
    unsafe {
        const LINK_OFF: u32 = 0xA80;
        const FIELD_OFF: u32 = 8;
        let linked = (this_ptr.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        f32::from_bits((linked.wrapping_add(FIELD_OFF) as *const u32).read_unaligned())
    }
});
