// original: 0x00b8c2f0 DISPLAY_TEXT_WITH_LITERAL_SUBSTRING
/// Native handler `DISPLAY_TEXT_WITH_LITERAL_SUBSTRING`: forwards two float coordinates and four integer handles to the engine.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
#[no_mangle]
pub extern "cdecl" fn rn24_display_text_with_literal_substring(ctx: u32) {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        lf_rn24_rt::callee_cdecl!(1, (), unsafe { (args as *const f32).read().to_bits() }, unsafe { (args.add(1) as *const f32).read().to_bits() }, unsafe { args.add(2).read() }, unsafe { args.add(3).read() }, unsafe { args.add(4).read() }, unsafe { args.add(5).read() });
}
