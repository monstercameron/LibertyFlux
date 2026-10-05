// original: 0x00beaac0 task_init_8a
/// Initialise a type-0x8a task object from a source record.
///
/// `this` takes the type id byte 0x8a at +0, the shared task global (file
/// VA 0x011735A4) at +4 and `arg0` at +8. Copy groups follow, gated on
/// flag bits in source byte +0x2d: bit 0x01: src+0x08 -> this+0x0c.
/// Returns the last value loaded (the source pointer when no group is
/// taken). Thiscall, two stack arguments (a word and the source pointer).
export!(thiscall, rw_00beaac0(this: u32, arg0: u32, src: u32) -> u32 {
    unsafe {
        const TYPE_ID: u8 = 0x8a;
        const TASK_GLOBAL: u32 = 0x011735A4;
        const SRC_FLAGS: u32 = 0x2d;
        let shared = (relocated(TASK_GLOBAL) as *const u32).read_unaligned();
        (this as *mut u8).write(TYPE_ID);
        ((this + 8) as *mut u32).write_unaligned(arg0);
        ((this + 4) as *mut u32).write_unaligned(shared);
        let flags = ((src + SRC_FLAGS) as *const u8).read();
        let mut last = src;
        if flags & 0x01 != 0 {
            let v = ((src + 0x08) as *const u32).read_unaligned();
            ((this + 0x0c) as *mut u32).write_unaligned(v);
            last = v;
        }
        last
    }
});
