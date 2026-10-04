// original: 0x005e80f0 CREATE_HTML_SCRIPT_OBJECT
/// Script native `CREATE_HTML_SCRIPT_OBJECT` (hash 0x6AA63375).
///
/// Forwards one script argument to the engine and stores its full 32-bit
/// answer (an object handle) into the return slot.
///
/// Note: the engine callee pops its own argument (the handler performs no
/// caller-side cleanup), so it is called with the stdcall convention. (An
/// earlier revision of this rewrite wrongly used cdecl; the checker proved
/// the callee pops by faulting the original on every trial.)
export!(cdecl, rw_005e80f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_stdcall!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
