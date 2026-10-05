// original: 0x00a0fbc0 flag_gated_dispatch (proposed)
/// Dispatch a typed object to the handler unless a flag excuses its type.
///
/// Does nothing when `obj` is null. Reads the flag byte at `ctx + 0x8e8`
/// and the type bits (6-9) of the word at `obj + 0x28`: returns without
/// calling when bit 0 is clear and the type is 0xc0, when bit 1 is clear
/// and the type is 0x80, or when bit 2 is clear and the type is 0x100.
/// Otherwise calls the handler as `handler(obj, extra)`. No return value
/// is set. Cdecl, three stack arguments.
export!(cdecl, rw_00a0fbc0(obj: u32, extra: u32, ctx: u32) -> u32 {
    unsafe {
        const HANDLER: u32 = 1;
        const TYPE_OFF: u32 = 0x28;
        const TYPE_MASK: u32 = 0x3c0;
        const FLAG_OFF: u32 = 0x8e8;
        if obj == 0 {
            return 0;
        }
        let flags = ((ctx + FLAG_OFF) as *const u8).read();
        let ty = ((obj + TYPE_OFF) as *const u32).read_unaligned() & TYPE_MASK;
        if flags & 1 == 0 && ty == 0xc0 {
            return 0;
        }
        if flags & 2 == 0 && ty == 0x80 {
            return 0;
        }
        if flags & 4 == 0 && ty == 0x100 {
            return 0;
        }
        callee_cdecl!(HANDLER, u32, obj, extra);
        0
    }
});
