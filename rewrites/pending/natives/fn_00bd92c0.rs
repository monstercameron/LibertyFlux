// original: 0x00bd92c0 SET_LCPD_COP_SCORE
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `SET_LCPD_COP_SCORE`: forwards one script word to the engine.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_set_lcpd_cop_score(ctx: u32) -> () {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        callee_cdecl!(1, (), unsafe { args.read() });
});
