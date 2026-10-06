// original: 0x0090a2a0 font_range_init (proposed)
/// Initialise a range object with two bounds and its vtable.
///
/// `this` points to the object. `lo` goes to word `+4`, `hi` to word `+8`,
/// word `+0` (the vtable pointer) is set to the range vtable and word `+0xc`
/// is zeroed. Returns `this`. Thiscall with two stack words, callee cleanup.
export!(thiscall, rw_0090a2a0(this: u32, lo: u32, hi: u32) -> u32 {
    unsafe {
        /// Range object vtable (file VA).
        const VTABLE: u32 = 0x00E85F48;
        ((this.wrapping_add(4)) as *mut u32).write_unaligned(lo);
        ((this.wrapping_add(8)) as *mut u32).write_unaligned(hi);
        (this as *mut u32).write_unaligned(relocated(VTABLE));
        ((this.wrapping_add(0x0c)) as *mut u32).write_unaligned(0);
        this
    }
});
