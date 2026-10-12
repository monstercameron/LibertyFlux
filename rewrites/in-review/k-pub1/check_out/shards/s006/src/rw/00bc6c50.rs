// original: 0x00bc6c50 IS_GARAGE_OPEN
use lf_k2_rt::{callee_cdecl, export};
// IS_GARAGE_OPEN: forward the garage id; store the answer's low byte
// through the return slot. Returns the slot.
export!(cdecl, rw_00BC6C50(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let open = __lf_callee_cdecl!(1, u32, *a) & 0xFF;
        let slot = ret_slot(ctx);
        *slot = open;
        slot as u32
    }
});

unsafe fn args_of(ctx: u32) -> *const u32 {
    * (ctx as *const u32).add(2) as *const u32
}

// Shared context helpers, verbatim from lane r-n20 (its out/ files use them).
unsafe fn ret_slot(ctx: u32) -> *mut u32 {
    *(ctx as *const u32) as *mut u32
}
