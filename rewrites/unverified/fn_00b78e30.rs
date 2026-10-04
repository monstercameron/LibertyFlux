// original: 0x00b78e30 task_header_init (proposed)

/// Initialise a task header and return it.
///
/// Stores the header's type tag, clears the next two words and the flags
/// word at `+0x10`, and clears the low three bits of the word at `+0x0C`,
/// keeping its other bits. Returns `this`.
///
/// Original: thiscall, no stack words, plain `ret`.
lf_checker_rt::export!(thiscall, rw_00b78e30(this: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x00EB2BD4;
        let tag = lf_checker_rt::relocated(TAG);
        (this as *mut u32).write_unaligned(tag);
        ((this + 4) as *mut u32).write_unaligned(0);
        ((this + 8) as *mut u32).write_unaligned(0);
        let flags = ((this + 0x0C) as *const u32).read_unaligned();
        ((this + 0x0C) as *mut u32).write_unaligned(flags & !7);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        this
    }
});
