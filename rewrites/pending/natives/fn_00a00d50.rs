// original: 0x00a00d50 GET_OBJECT_FRAGMENT_DAMAGE_HEALTH
/// Script native `GET_OBJECT_FRAGMENT_DAMAGE_HEALTH` (hash 0x79CA30B1).
///
/// Forwards an object handle and a stack-slot-coerced boolean flag, then
/// stores the engine's single-precision answer (returned on the x87 stack)
/// into the return slot. Returns the slot pointer.
export!(cdecl, rw_00a00d50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let value: f32 = callee_cdecl!(1, f32, *args, quirked);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = value.to_bits();
        slot as u32
    }
});
