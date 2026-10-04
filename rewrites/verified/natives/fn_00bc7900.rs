// original: 0x00bc7900 SET_ENGINE_HEALTH
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `SET_ENGINE_HEALTH`: forwards one integer handle and one float value to the engine.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_set_engine_health(ctx: u32) -> () {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        callee_cdecl!(1, (), unsafe { args.read() }, unsafe { (args.add(1) as *const f32).read().to_bits() });
});
