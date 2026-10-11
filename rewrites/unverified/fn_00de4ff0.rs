// original: 0x00DE4FF0 scrolling_menu_clamp_visible_index
/// Form an unsigned threshold from the row words at +0x1F4 and +0x1FC, then
/// query the child interface at +0x1E0 through virtual slot +0x1D4. If the
/// threshold is below the returned count, return the threshold; otherwise
/// query again and return the second count minus one. Thiscall uses no stack
/// arguments.
lf_checker_rt::export!(thiscall, rw_de4ff0(this: u32) -> u32 {
    const CHILD: u32 = 0x1E0;
    const INDEX: u32 = 0x1F4;
    const BASE: u32 = 0x1FC;
    const METHOD: u32 = 0x1D4;
    unsafe {
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let threshold = ((this + INDEX) as *const u32)
            .read_unaligned()
            .wrapping_sub(1)
            .wrapping_add(((this + BASE) as *const u32).read_unaligned());
        let table = (child as *const u32).read_unaligned();
        let address = ((table + METHOD) as *const u32).read_unaligned();
        let method: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(address as usize);
        let first_count = method(child);
        if threshold < first_count {
            threshold
        } else {
            method(child).wrapping_sub(1)
        }
    }
});

