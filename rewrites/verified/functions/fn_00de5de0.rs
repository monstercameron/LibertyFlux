// original: 0x00DE5DE0 UIScrollingMenu::vf108
/// Forward the menu's child interface at +0x1E0 to virtual slot +0x1B0 and
/// return the method's 32-bit result. Thiscall passes the child object in
/// ECX and uses no stack arguments.
lf_checker_rt::export!(thiscall, rw_de5de0(this: u32) -> u32 {
    const CHILD: u32 = 0x1E0;
    const METHOD: u32 = 0x1B0;
    unsafe {
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let table = (child as *const u32).read_unaligned();
        let address = ((table + METHOD) as *const u32).read_unaligned();
        let method: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(address as usize);
        method(child)
    }
});

