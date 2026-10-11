// original: 0x00DE5030 UIGTAScrollingRow::vf109
/// Ask the row's child interface at +0x1E0 to run virtual slot +0x24 with a
/// zero stack argument, then forward the row to the base routine. The child
/// call's result is ignored; the base routine's result is returned.
lf_checker_rt::export!(thiscall, rw_de5030(this: u32) -> u32 {
    const CHILD: u32 = 0x1E0;
    const METHOD: u32 = 0x24;
    unsafe {
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let table = (child as *const u32).read_unaligned();
        let address = ((table + METHOD) as *const u32).read_unaligned();
        let method: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(address as usize);
        let _ = method(child, 0);
    }
    lf_checker_rt::callee_thiscall!(2, u32, this)
});

