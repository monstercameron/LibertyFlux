// original: 0x00b9e870 CREATE_RANDOM_FEMALE_CHAR
use lf_k2_rt::{callee_cdecl, export};
/// Script native `CREATE_RANDOM_FEMALE_CHAR`.
/// Forwards four arguments (three float coordinates passed through
/// verbatim as bits, one integer) to the ped engine function. No script
/// return value.
export!(cdecl, rw_00b9e870(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        // No script return: EAX keeps the engine answer.
        callee_cdecl!(1, u32, a0, a1, a2, a3)
    }
});
