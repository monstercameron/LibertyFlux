// original: 0x00bc64d0 GET_VEHICLE_ENGINE_REVS
/// Script native `GET_VEHICLE_ENGINE_REVS` (hash 0x2FFA0249).
///
/// Forwards one script argument (a vehicle handle) to the engine, which returns
/// a float in the x87 top-of-stack register; the handler stores it into
/// the return slot (the engine revs). Bits are forwarded untouched, so the
/// conversion is bit-exact by construction.
export!(cdecl, rw_00bc64d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: f32 = callee_cdecl!(1, f32, *args);
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = answer;
        slot as u32
    }
});

