// original: 0x00DE6A40 scrolling_menu_dispatch_slot_0x238
/// Forward the menu's child interface at +0x1E0 to virtual slot +0x238 and
/// return its 32-bit result. The method is thiscall and the target takes no
/// stack arguments.
lf_checker_rt::export!(thiscall, rw_de6a40(this: u32) -> u32 {
    const CHILD: u32 = 0x1E0;
    const METHOD: u32 = 0x238;
    unsafe {
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let table = (child as *const u32).read_unaligned();
        let address = ((table + METHOD) as *const u32).read_unaligned();
        let method: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(address as usize);
        method(child)
    }
});

