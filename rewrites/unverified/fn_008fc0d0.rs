// original: 0x008FC0D0 text_set_field_9a4
/// Store a word argument into the field at `+0x9a4`.
///
/// Returns the stored word. Thiscall, one stack argument.
export!(thiscall, rw_008fc0d0(this: u32, v: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0x9a4;
        ((this + FIELD) as *mut u32).write_unaligned(v);
        v
    }
});
