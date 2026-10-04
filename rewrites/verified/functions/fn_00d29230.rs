// original: 0x00d29230 CPedTargetting::vf6 (symbols)

/// Release the reference in slot `index` of the handle table at `+0x250`.
///
/// When the table word for `index` is nonzero, passes its address to the
/// release helper (intercepted) and clears it. No bounds check: `index` must
/// select one of the table's words. The original leaves `eax` untouched, so
/// no return channel is compared.
///
/// Original: 0x00D29230 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d29230(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x250;
        let slot = this + TABLE_BASE + index.wrapping_mul(4);
        if unsafe { (slot as *const u32).read_unaligned() } != 0 {
            let _: u32 = lf_checker_rt::callee_stdcall!(1, u32, slot);
            unsafe { (slot as *mut u32).write_unaligned(0) };
        }
        0
    }
});
