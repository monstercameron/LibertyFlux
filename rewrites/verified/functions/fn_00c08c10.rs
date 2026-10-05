// original: 0x00c08c10 stream_slot_configure (proposed)

/// Configure one streaming slot through the shared helper, then clear its flag.
///
/// `this` points to the slot; the helper (callee 1) receives the slot's data
/// area at `DATA`, the caller's `arg`, and the constant `MODE`. After the call
/// the flag byte at `FLAG` is cleared. Returns whatever the helper returned.
///
/// Original: 0x00c08c10 (thiscall, one stack word; helper is cdecl, 3 args).
lf_checker_rt::export!(thiscall, rw_00c08c10(this: u32, arg: u32) -> u32 {
    unsafe {
        const DATA: u32 = 0x0c;
        const FLAG: u32 = 0x10b;
        const MODE: u32 = 0xff;
        const HELPER: u32 = 1;
        let r: u32 = lf_checker_rt::callee_cdecl!(HELPER, u32, this.wrapping_add(DATA), arg, MODE);
        (this.wrapping_add(FLAG) as *mut u8).write(0);
        r
    }
});
