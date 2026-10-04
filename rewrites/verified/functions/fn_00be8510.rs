// original: 0x00BE8510 timer_format_seconds (proposed)
/// Format a clamped counter into the object's text buffer.
///
/// `obj` points to the object and `arg` is a formatted only when its low
/// 16 bits are 0 or 1; any larger half-word returns 0 with no call. The
/// counter is the dword behind the pointer at `+0x00`, clamped at zero
/// (a negative value formats as 0), and is formatted with "%d" into the
/// buffer at `+0x16` through the format callee (cdecl, buffer, format,
/// value). Returns the callee's answer, or 0 when skipped.
///
/// Original: 0x00BE8510 (thiscall, one stack word). Only the argument's
/// low word gates the call, so full-word values with a small low half
/// still format.
lf_checker_rt::export!(thiscall, rw_00BE8510(obj: u32, arg: u32) -> u32 {
    unsafe {
        const COUNTER_PTR: u32 = 0x00;
        const TEXT: u32 = 0x16;
        const FMT: u32 = 0x00EB9648;
        const FORMAT: u32 = 1;
        let slot = ((obj + COUNTER_PTR) as *const u32).read_unaligned();
        let raw = (slot as *const i32).read_unaligned();
        let value = if raw < 0 { 0u32 } else { raw as u32 };
        if (arg as u16) > 1 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(
            FORMAT,
            u32,
            obj.wrapping_add(TEXT),
            lf_checker_rt::relocated(FMT),
            value
        )
    }
});

