// original: 0x005e6cb0 CREATE_EMERGENCY_SERVICES_CAR_RETURN_DRIVER
use lf_k2_rt::{callee_cdecl, export};
/// Script native `CREATE_EMERGENCY_SERVICES_CAR_RETURN_DRIVER`.
/// Forwards seven arguments (handle, three float coordinates passed
/// through verbatim as bits, three integers) to the vehicle engine
/// function and writes its low result byte to the script return slot.
export!(cdecl, rw_005e6cb0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        let a4 = *args.add(4);
        let a5 = *args.add(5);
        let a6 = *args.add(6);
        let ret_slot = *ctx_words as *mut u32;
        let answer: u32 = callee_cdecl!(1, u32, a0, a1, a2, a3, a4, a5, a6);
        // The handler keeps only the low byte (movzx).
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
