// original: 0x005e7f40 GET_WEB_PAGE_HEIGHT
use lf_k2_rt::{callee_addr, export};
// Rewrite of native handler GET_WEB_PAGE_HEIGHT.
//
// Passes page handle; stores the engine's floating-point answer.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the return-slot pointer, matching the value the original leaves in EAX.
export!(cdecl, rw_005e7f40(ctx: u32) -> u32 {
    unsafe {
        let argv = *((ctx + 8) as *const u32) as *const u32;
        let a0 = *argv.add(0);
        let ret = *(ctx as *const u32) as *mut f32;
        let engine: extern "cdecl" fn(u32) -> f32 =
            core::mem::transmute(callee_addr(1) as usize);
        *ret = engine(a0);
        ret as u32
    }
});
