// original: 0x00ba0c90 REGISTER_TARGET
/// Native handler `REGISTER_TARGET`: forwards two script words to the engine.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
#[no_mangle]
pub extern "cdecl" fn rn24_register_target(ctx: u32) {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        lf_rn24_rt::callee_cdecl!(1, (), unsafe { args.read() }, unsafe { args.add(1).read() });
}
