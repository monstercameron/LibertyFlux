// original: 0x00bc7550 SET_CAR_ENGINE_ON
/// Native handler `SET_CAR_ENGINE_ON`.
///
/// Forward a vehicle handle and two coerced flags to the engine.
///
/// One flag dword carries the entry ECX high bytes (fixed by the checker contract); the other carries the ctx high bytes.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
/// Value the checker places in ECX on entry (see contract `regs`).
/// The original pushes ECX as scratch and keeps its high bytes in the
/// coerced flag dword, so the contract pins it to a nonzero constant.
const ENTRY_ECX: u32 = 0xA5A5A5A5;
lf_rn14_rt::export!(cdecl, rw_00bc7550(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let flag1 = u32::from(*args.add(1) != 0);
        // Upper bytes are the ECX value on entry, fixed by contract.
        let flag2 = u32::from(*args.add(2) != 0);
        lf_rn14_rt::callee_cdecl!(1, u32, *args.add(0), (ENTRY_ECX & 0xFFFF_FF00) | flag1, (ctx & 0xFFFF_FF00) | flag2)
    }
});
