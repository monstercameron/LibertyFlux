// original: 0x0090a340 font_counter_delete_flag (proposed)
/// Set the counter object's virtual table, freeing it when the flag asks.
///
/// `this` points to the counter object. Its first word (the vtable pointer)
/// is always set to the counter vtable. When bit 0 of `flag` is set the
/// object is then passed to the deleting helper (the `0x401250` callee);
/// otherwise the object is left in place. Returns `this` in both cases.
/// Thiscall with one stack word, callee cleanup.
export!(thiscall, rw_0090a340(this: u32, flag: u32) -> u32 {
    unsafe {
        /// Counter object vtable (file VA).
        const VTABLE: u32 = 0x00E85F28;
        (this as *mut u32).write_unaligned(relocated(VTABLE));
        if flag & 1 != 0 {
            let _: u32 = callee_cdecl!(1, u32, this);
        }
        this
    }
});
