// original: 0x00bd8d00 NETWORK_SET_SCRIPT_LOBBY_STATE
/// Forward the coerced lobby-state flag to the engine function.
///
/// Same stack-slot boolean coercion as the lock-on handler: the pushed
/// dword is the context address with its low byte replaced by the flag.
export!(cdecl, rw_00bd8d00(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let state = unsafe { *args.add(0) };
    // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
    let coerced = ((state != 0) as u32);
    callee_cdecl!(1, u32, coerced)
});
