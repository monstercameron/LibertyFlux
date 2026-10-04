// original: 0x00bc6100 GET_PETROL_TANK_HEALTH
/// Script native `GET_PETROL_TANK_HEALTH` (hash 0x2C835642).
///
/// Forwards one script argument (a vehicle handle) to the engine, which
/// answers with a float on the x87 stack; the handler pops it into the
/// return slot.
export!(cdecl, rw_00bc6100(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, f32, *args);
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = answer;
        slot as u32
    }
});
