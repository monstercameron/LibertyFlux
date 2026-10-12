// original: 0x00C85C40 gtaFragType::destroy_lights

/// Reset the receiver's vtable, destroy each entry in its counted list through
/// the entry's first virtual slot, notify the list helper, and tail-call the
/// base fragment routine. The list pointer is at `+0x210`, and the unsigned
/// 16-bit entry count is at `+0x216`; entries are spaced `0x6C` bytes apart.
/// The virtual cleanup receives the entry as `this` and a zero stack argument.
lf_checker_rt::export!(thiscall, rw_00c85c40(this: u32) -> u32 {
    const LIST_POINTER_OFFSET: u32 = 0x210;
    const ENTRY_COUNT_OFFSET: u32 = 0x216;
    const ENTRY_STRIDE: u32 = 0x6C;
    const FRAGMENT_VTABLE: u32 = 0x00ED_63BC;
    const DESTROY_ENTRY: u32 = 1;
    const NOTIFY_LIST: u32 = 2;
    const DESTROY_BASE: u32 = 3;

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(FRAGMENT_VTABLE));
    }
    let entries = unsafe {
        (this.wrapping_add(LIST_POINTER_OFFSET) as *const u32).read_unaligned()
    };
    let entry_count = unsafe {
        (this.wrapping_add(ENTRY_COUNT_OFFSET) as *const u16).read_unaligned() as u32
    };

    if entry_count != 0 {
        for index in 0..entry_count {
            let entry = entries.wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let vtable = unsafe { (entry as *const u32).read_unaligned() };
            let destroy_address = unsafe { (vtable as *const u32).read_unaligned() };
            let destroy_entry: unsafe extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(destroy_address) };
            let _ = unsafe { destroy_entry(entry, 0) };
        }
        let _ = lf_checker_rt::callee_cdecl!(NOTIFY_LIST, u32, entries);
    }

    let _ = DESTROY_ENTRY;
    lf_checker_rt::callee_thiscall!(DESTROY_BASE, u32, this)
});
