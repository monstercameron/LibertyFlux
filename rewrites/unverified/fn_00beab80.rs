// original: 0x00beab80 task_init_8c
/// Initialise a type-0x8c task object from a source record.
///
/// `this` takes the type id byte 0x8c at +0, the shared task global (file
/// VA 0x011735A4) at +4 and `arg0` at +8. Copy groups follow, gated on
/// flag bits in source byte +0x2d: bit 0x02: src+0x0c -> this+0x0c.
/// Returns the last value loaded (the shared global when no group is
/// taken). Thiscall, two stack arguments (a word and the source pointer).
export!(thiscall, rw_00beab80(this: u32, arg0: u32, src: u32) -> u32 {
    unsafe {
        const TYPE_ID: u8 = 0x8c;
        const TASK_GLOBAL: u32 = 0x011735A4;
        const SRC_FLAGS: u32 = 0x2d;
        let shared = (relocated(TASK_GLOBAL) as *const u32).read_unaligned();
        (this as *mut u8).write(TYPE_ID);
        ((this + 8) as *mut u32).write_unaligned(arg0);
        ((this + 4) as *mut u32).write_unaligned(shared);
        let flags = ((src + SRC_FLAGS) as *const u8).read();
        let mut last = shared;
        if flags & 0x02 != 0 {
            let v = ((src + 0x0c) as *const u32).read_unaligned();
            ((this + 0x0c) as *mut u32).write_unaligned(v);
            last = v;
        }
        last
    }
});
