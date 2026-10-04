// original: 0x00b8c170 DISPLAY_TEXT
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `DISPLAY_TEXT`: forwards two float coordinates and one integer handle to the engine.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_display_text(ctx: u32) -> () {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        callee_cdecl!(1, (), unsafe { (args as *const f32).read().to_bits() }, unsafe { (args.add(1) as *const f32).read().to_bits() }, unsafe { args.add(2).read() });
});
