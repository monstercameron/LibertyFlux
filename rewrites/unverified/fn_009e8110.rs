// original: 0x009e8110 ped_forward_link_tag
/// Forwards the tag word at `[this+0xA80]+0x3C` to the shared
/// lookup and returns its answer. (thiscall.)
lf_checker_rt::export!(thiscall, rw_009e8110(this_ptr: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0xA80;
        const TAG_OFF: u32 = 0x3C;
        let linked = (this_ptr.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        let tag = (linked.wrapping_add(TAG_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(1, u32, tag)
    }
});
