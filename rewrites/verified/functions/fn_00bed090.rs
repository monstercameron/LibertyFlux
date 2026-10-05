// original: 0x00bed090 set_type_0f
/// Tag an object with type id 0x0f and store one argument word.
///
/// Writes byte 0x0f to `[this]` and the argument to `[this+4]`. Returns the
/// argument (the original's EAX leftover). Thiscall, one stack argument.
export!(thiscall, rw_00bed090(this: u32, arg: u32) -> u32 {
    unsafe {
        const TYPE_ID: u8 = 0x0f;
        ((this) as *mut u8).write(TYPE_ID);
        ((this + 4) as *mut u32).write_unaligned(arg);
        arg
    }
});
