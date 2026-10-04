// original: 0x00BA2310 SET_PED_ALLOW_MISSION_ONLY_DRIVEBY_USE
// Rewrite of native handler SET_PED_ALLOW_MISSION_ONLY_DRIVEBY_USE.
//
// Passes ped handle plus a 0/1 flag to the driveby-flag engine helper.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// The flag argument reproduces the original's codegen quirk: the 0/1 byte
// is written over the low byte of the incoming context-pointer slot, so the
// engine observes `(ctx & !0xFF) | flag` and reads only the low byte.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00ba2310(ctx: u32) -> u32 {
    unsafe {
        let argv = *((ctx + 8) as *const u32) as *const u32;
        let a0 = *argv.add(0);
        let a1 = *argv.add(1);
        let engine: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        engine(a0, (ctx & 0xFFFFFF00) | ((a1 != 0) as u32))
    }
});
