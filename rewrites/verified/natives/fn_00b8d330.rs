// original: 0x00B8D330 PRINT_WITH_NUMBER
use lf_k2_rt::{callee_cdecl, export, relocated};

/// PRINT_WITH_NUMBER: show a text with numbers.
///
/// Native handler. Forwards text id plus three numbers to the text engine.
export!(cdecl, rw_00b8d330(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        callee_cdecl!(1, u32, a0, a1, a2, a3)
    }
});
