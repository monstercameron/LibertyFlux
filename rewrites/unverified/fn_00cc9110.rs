// original: 0x00CC9110 fragInstNMGta::vf30

/// If the nested record at receiver offset `+0x0c` has not set its `0x20`
/// marker at `+0xf4`, calls vtable slot `+0xe0` on the receiver and sets that
/// marker when the query returns nonzero. It then calls the common update
/// helper with the receiver and the one original stack argument, returning the
/// helper's EAX. The nested record and virtual target are supplied by the
/// caller; both helper bodies are represented by scripted checker callees.
lf_checker_rt::export!(thiscall, rw_00cc9110(this: u32, argument: u32) -> u32 {
    const NESTED_RECORD: u32 = 0x0c;
    const VTABLE_SLOT: u32 = 0xe0;
    const MARKER: u32 = 0xf4;
    const MARKER_BIT: u8 = 0x20;
    const VTABLE_QUERY: u32 = 1;
    const UPDATE_HELPER: u32 = 2;

    let nested = unsafe { (this.wrapping_add(NESTED_RECORD) as *const u32).read_unaligned() };
    let marker = nested.wrapping_add(MARKER) as *mut u8;
    let marker_flags = unsafe { marker.read() };
    if marker_flags & MARKER_BIT == 0 {
        let vtable = unsafe { (this as *const u32).read_unaligned() };
        let method_address = unsafe {
            (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned()
        };
        let query: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(method_address as usize) };
        if query(this) != 0 {
            unsafe { marker.write(marker_flags | MARKER_BIT) };
        }
    }

    lf_checker_rt::callee_thiscall!(UPDATE_HELPER, u32, this, argument)
});
