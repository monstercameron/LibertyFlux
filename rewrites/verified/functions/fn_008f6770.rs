// original: 0x008F6770 Input_GetDeviceState

/// Return `this + STATE_OFF` when `kind` equals the device kind word at
/// `+KIND_OFF` or the wildcard `ANY_KIND`; otherwise return null.
///
/// Arguments: `this` (ECX) points to the device record, `kind` is an exact
/// 32-bit match value (equality only, no signedness involved). Reads one
/// dword, writes nothing. Convention: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_008f6770(this: u32, kind: u32) -> u32 {
    unsafe {
        const ANY_KIND: u32 = 2;
        const KIND_OFF: u32 = 4;
        const STATE_OFF: u32 = 8;
        const ANY_KIND: u32 = 2;
        let cur = (this.wrapping_add(KIND_OFF) as *const u32).read_unaligned();
        if kind == cur || kind == ANY_KIND {
            this.wrapping_add(STATE_OFF)
        } else {
            0
        }
    }
});
