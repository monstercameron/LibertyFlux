// original: 0x00BD95A0 SET_RICH_PRESENCE_TEMPLATEMP4
use lf_k2_rt::{callee_cdecl, export, relocated};

/// SET_RICH_PRESENCE_TEMPLATEMP4: set a rich-presence template.
///
/// Native handler. Forwards two template words to the presence engine.
export!(cdecl, rw_00bd95a0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        callee_cdecl!(1, u32, a0, a1)
    }
});
