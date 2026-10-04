// original: 0x00bc70a0 OPEN_GARAGE
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `OPEN_GARAGE`: forwards one script word to the engine.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_open_garage(ctx: u32) -> () {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        callee_cdecl!(1, (), unsafe { args.read() });
});
