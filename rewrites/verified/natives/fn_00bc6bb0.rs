// original: 0x00bc6bb0 IS_COP_VEHICLE_IN_AREA_3D_NO_SAVE
/// Script native `IS_COP_VEHICLE_IN_AREA_3D_NO_SAVE` (hash 0x72F81072).
///
/// Forwards six float bit-patterns (two opposite corners of a 3D box) to
/// the engine and stores the low byte of its answer (zero-extended) into
/// the return slot. The exit value is the slot pointer.
export!(cdecl, rw_00bc6bb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
