// original: 0x0065fc90 gamer_record_copy
/// Copy a gamer record into this task object.
///
/// Copies the two header words, delegates the embedded string block at
/// offset 8 to the shared copy helper, copies the trailing scalar fields
/// (dwords at `0x78`/`0x80`/`0x88`/`0x8c`, words at `0x7c`/`0x84`), then
/// copies the 0x200-byte blob at `0x90` verbatim. Returns `this`.
export!(thiscall, rw_0065fc90(this: u32, src: u32) -> u32 {
    unsafe {
        ((this) as *mut u32).write((src as *const u32).read());
        ((this + 4) as *mut u32).write(((src + 4) as *const u32).read());
        callee_thiscall!(1, u32, this + 8, src + 8);
        ((this + 0x78) as *mut u32).write(((src + 0x78) as *const u32).read());
        ((this + 0x7c) as *mut u16).write(((src + 0x7c) as *const u16).read());
        ((this + 0x80) as *mut u32).write(((src + 0x80) as *const u32).read());
        ((this + 0x84) as *mut u16).write(((src + 0x84) as *const u16).read());
        ((this + 0x88) as *mut u32).write(((src + 0x88) as *const u32).read());
        ((this + 0x8c) as *mut u32).write(((src + 0x8c) as *const u32).read());
        core::ptr::copy_nonoverlapping(
            (src + 0x90) as *const u8,
            (this + 0x90) as *mut u8,
            0x200,
        );
        this
    }
});

