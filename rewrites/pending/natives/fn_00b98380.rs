// original: 0x00b98380 GET_PAD_PITCH_ROLL
/// Script native `GET_PAD_PITCH_ROLL` (hash 0x767B7EC9).
///
/// Forwards three script arguments (a pad index and two out-pointers) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b98380(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2),);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
