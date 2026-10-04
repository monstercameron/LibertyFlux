// original: 0x00b9f8d0 IS_CHAR_IN_ANGLED_AREA_3D
/// Script native handler `IS_CHAR_IN_ANGLED_AREA_3D`.
///
/// Checks if the character is within the angled 3D area.
///
/// Forwards 9 script arguments to the engine function and stores the
/// low byte of its answer (zero-extended) into the return slot.
/// Float arguments pass through by value as raw bits (bit-exact copies).
/// Argument 8 is bool-coerced (`!= 0`); the original also overwrites the low
/// byte of its own incoming stack slot with the flag, which is dead after
/// return and therefore not reproduced (see contract note).
/// handler function: `0x00b9f8d0`, engine call site: `0x00b9f936`.
export!(cdecl, rw_b9f8d0(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let ped = *args.add(0);
        let x1 = *args.add(1);
        let y1 = *args.add(2);
        let z1 = *args.add(3);
        let x2 = *args.add(4);
        let y2 = *args.add(5);
        let z2 = *args.add(6);
        let angle = *args.add(7);
        let use_z = ((*args.add(8) != 0) as u32) | ((ctx as u32) & 0xFFFF_FF00);
        let answer: u32 = callee_cdecl!(1, u32, ped, x1, y1, z1, x2, y2, z2, angle, use_z);
        *(*ctx).ret_ptr = answer & 0xFF;
        (*ctx).ret_ptr as u32
    }
});
