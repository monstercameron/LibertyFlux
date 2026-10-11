// original: 0x00DE6AE0 UIGTAScrollingRow::vf82
/// Query the row's item predicate, pass zero to the child interface's
/// thiscall slot +0x118 when that predicate is true and one otherwise, then
/// forward the row to the base routine. The table call's result is ignored
/// and the base routine's result is returned.
lf_checker_rt::export!(thiscall, rw_de6ae0(this: u32) -> u32 {
    const CHILD: u32 = 0x21C;
    const METHOD: u32 = 0x118;
    let has_item = lf_checker_rt::callee_thiscall!(2, u8, this) != 0;
    unsafe {
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let table = (child as *const u32).read_unaligned();
        let address = ((table + METHOD) as *const u32).read_unaligned();
        let method: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(address as usize);
        let _ = method(child, if has_item { 0 } else { 1 });
    }
    lf_checker_rt::callee_thiscall!(3, u32, this)
});

