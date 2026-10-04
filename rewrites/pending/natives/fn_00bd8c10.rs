// original: 0x00BD8C10 NETWORK_SET_FRIENDLY_FIRE_OPTION
// Rewrite of native handler NETWORK_SET_FRIENDLY_FIRE_OPTION.
//
// Passes flag word coerced to a 0/1 byte for the friendly-fire engine helper.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// The flag argument reproduces the original's codegen quirk: the 0/1 byte
// is written over the low byte of the incoming context-pointer slot, so the
// engine observes `(ctx & !0xFF) | flag` and reads only the low byte.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00bd8c10(ctx: u32) -> u32 {
    unsafe {
        let argv = *((ctx + 8) as *const u32) as *const u32;
        let a0 = *argv.add(0);
        let engine: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        engine((ctx & 0xFFFFFF00) | ((a0 != 0) as u32))
    }
});
