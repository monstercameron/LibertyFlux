// original: 0x00bc7d90 SET_TRAIN_STOPS_FOR_STATIONS
/// Set whether trains stop at stations.
///
/// Forwards the train handle (argument 0) and the flag (argument 1, normalised
/// to 0/1) to the engine. The pushed flag dword carries the context pointer's
/// high bytes (see `rw_00b9ebf0`), reproduced exactly. Returns whatever the
/// engine returned.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00bc7d90(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let pushed = flag;
        callee_cdecl!(1, u32, *args, pushed)
    }
});
