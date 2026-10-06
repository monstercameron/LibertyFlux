// original: 0x00872310 rage::crExpressionProcessor::ExpressionFilter::vf0
/// Deleting destructor of an expression filter.
///
/// Clears the filter table pointer at `this + 0x0C`, stamps the base-class
/// table, and when the low bit of the flags argument is set frees `this`
/// through the thread-local manager (TLS slot 0 points at the thread block,
/// whose word at `+8` is the manager object; the free routine is vtable
/// slot `0x0C`, taking the block as its stack argument). Returns `this`.
///
/// Original: 0x00872310 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00872310(this: u32, flags: u32) -> u32 {
    unsafe {
        const FILTER_OFF: u32 = 0x0C;
        const VTABLE: u32 = 0xE86AFC;
        const FREE_FLAG: u32 = 1;
        const MANAGER_OFF: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        ((this + FILTER_OFF) as *mut u32).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if flags & FREE_FLAG != 0 {
            let thread = lf_checker_rt::tls_slot(0);
            let manager = ((thread + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable + FREE_SLOT) as *const u32).read_unaligned();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free(manager, this);
        }
    }
    this
});
