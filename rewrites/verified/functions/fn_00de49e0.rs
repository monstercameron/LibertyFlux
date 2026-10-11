// original: 0x00DE49E0 scrolling_row_compare_cursor_to_count
/// If the row is enabled, obtain the child interface at +0x1E0, call its
/// virtual method at byte offset +0x1D4, then call the row helper. Return one
/// when the helper's unsigned result is below the first result minus one;
/// return zero for a disabled row. The method uses thiscall and observes the
/// two calls in that order.
lf_checker_rt::export!(thiscall, rw_de49e0(this: u32) -> u8 {
    const DISABLED: u32 = 0x214;
    const CHILD: u32 = 0x1E0;
    const VTABLE: u32 = 0;
    const METHOD: u32 = 0x1D4;
    unsafe {
        if ((this + DISABLED) as *const u8).read() != 0 {
            return 0;
        }
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let table = (child as *const u32).read_unaligned();
        let address = ((table + METHOD) as *const u32).read_unaligned();
        let method: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(address as usize);
        let current_count = method(child);
        let helper_count = lf_checker_rt::callee_thiscall!(2, u32, this);
        u8::from(helper_count < current_count.wrapping_sub(1))
    }
});

