// original: 0x005e81d0 CONVERT_THEN_ADD_STRING_TO_HTML_SCRIPT_OBJECT
/// Script native `CONVERT_THEN_ADD_STRING_TO_HTML_SCRIPT_OBJECT` (hash 0x72EC0AA6).
///
/// Probes a script-string handle through the engine converter: if the
/// probe reports failure (low byte zero) the handler returns that
/// answer unchanged, otherwise it converts the string, wraps the
/// result with two zero words, and appends it to the HTML object
/// named by the other argument. No return slot is written.
export!(cdecl, rw_005e81d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let this = relocated(0x116BFF0);
        let probe = callee_thiscall!(1, u32, this, *args.add(1));
        if probe & 0xFF == 0 {
            return probe;
        }
        let converted = callee_thiscall!(2, u32, this, *args.add(1));
        let prepared = callee_cdecl!(3, u32, converted, 0, 0);
        callee_stdcall!(4, u32, *args, prepared)
    }
});
