// original: 0x00b9ff70 IS_COP_PED_IN_AREA_3D_NO_SAVE
/// Script native `IS_COP_PED_IN_AREA_3D_NO_SAVE` (hash 0x01866CB5).
///
/// Forwards six float bit-patterns (two 3D corners of a box) to the
/// engine and stores the low byte of its answer (zero-extended) into
/// the return slot. Floats are copied as raw bits, so the forward is
/// bit-exact.
export!(cdecl, rw_00b9ff70(ctx: *const u8) -> u32 {
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
