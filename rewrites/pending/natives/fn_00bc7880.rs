// original: 0x00bc7880 SET_CAR_WATERTIGHT
/// Script native handler `SET_CAR_WATERTIGHT`.
///
/// Sets whether the vehicle is watertight.
///
/// Forwards 2 script arguments to the engine function; no return slot.
/// Argument 1 is bool-coerced (`!= 0`); the original also overwrites the low
/// byte of its own incoming stack slot with the flag, which is dead after
/// return and therefore not reproduced (see contract note).
/// handler function: `0x00bc7880`, engine call site: `0x00bc7896`.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_bc7880(ctx: *const NativeCtx03) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let vehicle = *args.add(0);
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let watertight = ((*args.add(1) != 0) as u32);
        callee_cdecl!(1, u32, vehicle, watertight)
    }
});
