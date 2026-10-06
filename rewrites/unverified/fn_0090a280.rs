// original: 0x0090a280 font_counter_init (proposed)
/// Initialise a counter object: set its vtable and zero its two fields.
///
/// `this` points to the object. Word `+0` (the vtable pointer) is set to
/// the counter vtable, words `+4` and `+8` are zeroed. Returns `this`.
/// Thiscall with no stack arguments.
export!(thiscall, rw_0090a280(this: u32) -> u32 {
    unsafe {
        /// Counter object vtable (file VA).
        const VTABLE: u32 = 0x00E85F38;
        (this as *mut u32).write_unaligned(relocated(VTABLE));
        ((this.wrapping_add(4)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(8)) as *mut u32).write_unaligned(0);
        this
    }
});
