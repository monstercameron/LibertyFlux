// original: 0x009e9620 ped_forward_to_inner
/// Forwards this object to virtual slot `+0x24` of the inner
/// object at `+0xA80`, returning its answer. (thiscall.)
lf_checker_rt::export!(thiscall, rw_009e9620(this_ptr: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0xA80;
        const SLOT: u32 = 0x24;
        let inner = (this_ptr.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        let vt = (inner as *const u32).read_unaligned();
        let slot = (vt.wrapping_add(SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        f(inner, this_ptr)
    }
});
