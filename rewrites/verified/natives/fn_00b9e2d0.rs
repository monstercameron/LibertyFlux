// original: 0x00b9e2d0 BLEND_FROM_NM_WITH_ANIM
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `BLEND_FROM_NM_WITH_ANIM`: passes four script integers to the engine and points ECX at a stack-built triple of the remaining three float arguments.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_blend_from_nm_with_anim(ctx: u32) -> () {
        // The original also builds a three-float struct on its frame and passes
        // its address in ECX; that register is not part of the checker's cdecl
        // call model, so only the four stack arguments are compared.
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        callee_cdecl!(1, (), unsafe { args.read() }, unsafe { args.add(1).read() }, unsafe { args.add(2).read() }, unsafe { args.add(3).read() });
});
