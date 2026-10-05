// original: 0x00beaaf0 task_init_8b_full
/// Initialise a type-0x8b task object: base part, header, five gated groups.
///
/// Calls the shared base initialiser first (thiscall, no stack arguments;
/// intercepted by the checker), which masks `[this+1]` with 0xe0. Then
/// writes the type id byte 0x8b at +0, the shared task global (file VA
/// 0x011735A4) at +4, `arg0` at +8, and copies the source half-word at +4
/// to `[this+2]`. Five copy groups follow, gated on source flag byte
/// +0x2d, each also setting its bit in `[this+1]`: bit 0x01: src+0x08 ->
/// this+0x0c; 0x02: src+0x0c -> +0x10; 0x04: src+0x10/0x14/0x18 ->
/// +0x14/+0x18/+0x1c; 0x08: src+0x1c -> +0x20; 0x10: src+0x20 -> +0x24.
/// Returns the last value loaded (the half-word merged under `arg0`'s high
/// half when no group is taken). Thiscall, two stack arguments.
export!(thiscall, rw_00beaaf0(this: u32, arg0: u32, src: u32) -> u32 {
    unsafe {
        const TYPE_ID: u8 = 0x8b;
        const TASK_GLOBAL: u32 = 0x011735A4;
        const SRC_FLAGS: u32 = 0x2d;
        const BASE_CTOR: u32 = 1;
        let _: u32 = callee_thiscall!(BASE_CTOR, u32, this);
        let shared = (relocated(TASK_GLOBAL) as *const u32).read_unaligned();
        (this as *mut u8).write(TYPE_ID);
        ((this + 8) as *mut u32).write_unaligned(arg0);
        ((this + 4) as *mut u32).write_unaligned(shared);
        let hw = ((src + 4) as *const u16).read_unaligned();
        ((this + 2) as *mut u16).write_unaligned(hw);
        let flags = ((src + SRC_FLAGS) as *const u8).read();
        let mut last = (arg0 & 0xFFFF0000) | hw as u32;
        if flags & 0x01 != 0 {
            let b = ((this + 1) as *const u8).read();
            ((this + 1) as *mut u8).write(b | 0x01);
            let v = ((src + 0x08) as *const u32).read_unaligned();
            ((this + 0x0c) as *mut u32).write_unaligned(v);
            last = v;
        }
        if flags & 0x02 != 0 {
            let b = ((this + 1) as *const u8).read();
            ((this + 1) as *mut u8).write(b | 0x02);
            let v = ((src + 0x0c) as *const u32).read_unaligned();
            ((this + 0x10) as *mut u32).write_unaligned(v);
            last = v;
        }
        if flags & 0x04 != 0 {
            let b = ((this + 1) as *const u8).read();
            ((this + 1) as *mut u8).write(b | 0x04);
            let v = ((src + 0x10) as *const u32).read_unaligned();
            ((this + 0x14) as *mut u32).write_unaligned(v);
            last = v;
            let v = ((src + 0x14) as *const u32).read_unaligned();
            ((this + 0x18) as *mut u32).write_unaligned(v);
            last = v;
            let v = ((src + 0x18) as *const u32).read_unaligned();
            ((this + 0x1c) as *mut u32).write_unaligned(v);
            last = v;
        }
        if flags & 0x08 != 0 {
            let b = ((this + 1) as *const u8).read();
            ((this + 1) as *mut u8).write(b | 0x08);
            let v = ((src + 0x1c) as *const u32).read_unaligned();
            ((this + 0x20) as *mut u32).write_unaligned(v);
            last = v;
        }
        if flags & 0x10 != 0 {
            let b = ((this + 1) as *const u8).read();
            ((this + 1) as *mut u8).write(b | 0x10);
            let v = ((src + 0x20) as *const u32).read_unaligned();
            ((this + 0x24) as *mut u32).write_unaligned(v);
            last = v;
        }
        last
    }
});
